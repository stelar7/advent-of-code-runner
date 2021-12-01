#pragma once

#include "types.hpp"

namespace Spek
{
	class Duration
	{
	public:
		Duration();
		Duration(const Duration& a_Duration);
		~Duration();

		Duration& operator=(const Duration& a_Duration);
		Duration& operator+=(const Duration& a_Duration);
		Duration& operator-=(const Duration& a_Duration);

		Duration operator+(const Duration& a_Duration) const;
		Duration operator-(const Duration& a_Duration) const;

		bool operator ==(const Duration& a_Duration) const;
		bool operator !=(const Duration & a_Duration) const;
		bool operator <(const Duration& a_Duration) const;
		bool operator >(const Duration& a_Duration) const;
		bool operator <=(const Duration & a_Duration) const;
		bool operator >=(const Duration& a_Duration) const;

		static Duration FromNanoseconds(i64 a_Nanoseconds);
		static Duration FromMicroseconds(i64 a_Microseconds);
		static Duration FromMilliseconds(i64 a_Milliseconds);
		static Duration FromSeconds(i64 a_Seconds);
		static Duration FromMinutes(i64 a_Minutes);

		static Duration FromNanosecondsF(f64 a_Nanoseconds);
		static Duration FromMicrosecondsF(f64 a_Microseconds);
		static Duration FromMillisecondsF(f64 a_Milliseconds);
		static Duration FromSecondsF(f64 a_Seconds);
		static Duration FromMinutesF(f64 a_Minutes);

		f64 ToSecF64() const;

	private:
		Duration(i64 a_Nanosec);

		i64 m_Value;
	};

	Duration GetTimeSinceStart();
}

Spek::Duration operator ""_ns(u64 a_Nanoseconds);
Spek::Duration operator""_us(u64 a_Microseconds);
Spek::Duration operator ""_ms(u64 a_Milliseconds);
Spek::Duration operator ""_sec(u64 a_Seconds);
Spek::Duration operator ""_min(u64 a_Minutes);