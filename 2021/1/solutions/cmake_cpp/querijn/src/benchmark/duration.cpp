#include "duration.hpp"

#include <chrono>

namespace Spek
{
	using Clock = std::chrono::high_resolution_clock;
	using NanoSecDuration = std::chrono::nanoseconds;

	static auto g_StartTime = Clock::now();
	Duration GetTimeSinceStart()
	{
		auto t_CurrentTime = Clock::now();
		return Duration::FromNanoseconds(NanoSecDuration(t_CurrentTime - g_StartTime).count());
	}

	Duration::Duration() :
		m_Value(0)
	{
	}

	Duration::Duration(i64 a_Value) :
		m_Value(a_Value)
	{
	}

	Duration::Duration(const Duration& a_Duration) :
		m_Value(a_Duration.m_Value)
	{
	}

	Duration::~Duration()
	{
	}

	Duration& Duration::operator =(const Duration& a_Duration) { m_Value = a_Duration.m_Value; return *this; }
	Duration& Duration::operator +=(const Duration& a_Duration) { m_Value += a_Duration.m_Value; return *this; }
	Duration& Duration::operator -=(const Duration& a_Duration) { m_Value -= a_Duration.m_Value; return *this; }

	Duration Duration::operator +(const Duration& a_Duration) const { return Duration(m_Value + a_Duration.m_Value); }
	Duration Duration::operator -(const Duration& a_Duration) const { return Duration(m_Value - a_Duration.m_Value); }

	bool Duration::operator ==(const Duration& a_Duration) const { return m_Value == a_Duration.m_Value; }
	bool Duration::operator !=(const Duration& a_Duration) const { return m_Value != a_Duration.m_Value; }
	bool Duration::operator <(const Duration& a_Duration) const { return m_Value < a_Duration.m_Value; }
	bool Duration::operator >(const Duration& a_Duration) const { return m_Value > a_Duration.m_Value; }
	bool Duration::operator <=(const Duration& a_Duration) const { return m_Value <= a_Duration.m_Value; }
	bool Duration::operator >=(const Duration& a_Duration) const { return m_Value >= a_Duration.m_Value; }

	Duration Duration::FromNanoseconds(i64 a_Nanoseconds) { return Duration(a_Nanoseconds); }
	Duration Duration::FromMicroseconds(i64 a_Microseconds) { return Duration(a_Microseconds * 1000); }
	Duration Duration::FromMilliseconds(i64 a_Milliseconds) { return Duration(a_Milliseconds * 1000 * 1000); }
	Duration Duration::FromSeconds(i64 a_Seconds) { return Duration(a_Seconds * 1000 * 1000 * 1000); }
	Duration Duration::FromMinutes(i64 a_Minutes) { return Duration(a_Minutes * 60 * 1000 * 1000 * 1000); }

	Duration Duration::FromNanosecondsF(f64 a_Nanoseconds) { return Duration(a_Nanoseconds); }
	Duration Duration::FromMicrosecondsF(f64 a_Microseconds) { return Duration(a_Microseconds * 1000.0); }
	Duration Duration::FromMillisecondsF(f64 a_Milliseconds) { return Duration(a_Milliseconds * 1000.0 * 1000.0); }
	Duration Duration::FromSecondsF(f64 a_Seconds) { return Duration(a_Seconds * 1000.0 * 1000.0 * 1000.0); }
	Duration Duration::FromMinutesF(f64 a_Minutes) { return Duration(a_Minutes * 60.0 * 1000.0 * 1000.0 * 1000.0); }

	f64 Duration::ToSecF64() const
	{
		return f64(m_Value) / (1000.0 * 1000.0 * 1000.0);
	}
}

Spek::Duration operator""_ns(u64 a_Value) { return Spek::Duration::FromNanoseconds(a_Value); }
Spek::Duration operator""_us(u64 a_Value) { return Spek::Duration::FromMicroseconds(a_Value); }
Spek::Duration operator""_ms(u64 a_Value) { return Spek::Duration::FromMilliseconds(a_Value); }
Spek::Duration operator""_sec(u64 a_Value) { return Spek::Duration::FromSeconds(a_Value); }
Spek::Duration operator""_min(u64 a_Value) { return Spek::Duration::FromMinutes(a_Value); }