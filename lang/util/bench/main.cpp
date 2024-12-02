#include <array>
#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;
using MeasureClock = std::chrono::high_resolution_clock;

double AsMilliseconds(std::chrono::nanoseconds duration)
{
	auto durationNs = std::chrono::duration_cast<std::chrono::nanoseconds>(duration);
	return durationNs.count() / 1000000.0;
}

std::string DoubleAsString(double a_value, int n)
{
	std::ostringstream out;
	out.precision(3);
	out << std::fixed << a_value;
	std::string result = std::move(out).str();
	if (result.size() > n)
		result.resize(n);

	return result;
}

std::string Trim(std::string_view str)
{
	const char* start = str.data();
	const char* end = start + str.size() - 1;
	while (start < end && (*start == ' ' || *start == '\n' || *start == '\t' || *start == '\r'))
		start++;

	while (end > start && (*start == ' ' || *start == '\n' || *start == '\t' || *start == '\r'))
		end--;

	return std::string(start, end + 1);
}


double MeasureRun(std::string_view command, std::string_view expectedOutput = "")
{
	std::array<char, 2048> buffer;
	std::string result;

	auto start = MeasureClock::now();
	std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(command.data(), "r"), pclose);
	if (!pipe)
		throw std::runtime_error("popen() failed!");

	while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr)
		result += buffer.data();
	auto end = MeasureClock::now();

	double duration = AsMilliseconds(end - start);
	
	// Compare outputs if necessary
	if (!expectedOutput.empty())
	{
		if (result != expectedOutput)
			return -1;
	}

	return duration;
}

int main(int argc, char** argv)
{
	using namespace std::chrono_literals;

	bool isBuild = argc >= 6;

	std::string language = argv[1];
	fs::path    rootFolder = argv[2];
	fs::path    inputFolder = argv[3];
	std::string application = argv[4];
	std::string buildScript = isBuild ? argv[5] : "";
	std::string author = rootFolder.filename().generic_string();

	// Measure build
	double buildDuration = 0;
	if (isBuild)
	{
		std::string buildCommand = buildScript + " \"" + rootFolder.string() + "\"";
		buildDuration = MeasureRun(buildCommand);
	}

	// For each input file
	// Get each file with .input as extension
	double totalDuration = 0;
	double count = 0;
	for (const auto& file : std::filesystem::directory_iterator { inputFolder })
	{
		if (file.path().extension() != ".input")
			continue;

		fs::path inputPath = inputFolder / file;
		fs::path outputFile = inputPath; outputFile.replace_extension(".output");

		std::ifstream t(outputFile);
		std::stringstream buffer; buffer << t.rdbuf();
		std::string fileContent = buffer.str();

		// Measure execution
		std::string command = (rootFolder / application).string() + " \"" + inputPath.string() + "\"";
		double currentDuration = MeasureRun(command, fileContent);
		if (currentDuration < 0)
		{
			printf("%-10s %-15s %s ❌\n", language.c_str(), author.c_str(), inputPath.string().c_str());
			return 1;
		}

		totalDuration += currentDuration;
		count += 1;
	}

	auto buildDurationStr = DoubleAsString(buildDuration, 8) + "ms";
	auto totalDurationStr = DoubleAsString(totalDuration, 8) + "ms";
	auto avgDurationStr =   DoubleAsString(totalDuration / count, 8) + "ms";

	printf("%-10s %-15s %-10s %-10s %-10s ✅\n", language.c_str(), author.c_str(), buildDurationStr.c_str(), totalDurationStr.c_str(), avgDurationStr.c_str());
	return 0;
}