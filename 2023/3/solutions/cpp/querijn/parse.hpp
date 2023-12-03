#define BENCHMARK 0 // NDEBUG

#include <cstdint>
#include <cstring>
#include <cstdio>

#if defined(_DEBUG)
#define assert(input) do { if (!(input)) { __debugbreak(); } } while(0)
#else
#define assert(input) do { ((void)(input)); } while(0)
#endif

#if defined(_MSC_VER)
#define FORCE_INLINE __forceinline
#elif defined(__GNUC__)
#define FORCE_INLINE inline __attribute__((always_inline))
#elif (__STDC_VERSION__ >= 199901L)
#define FORCE_INLINE INLINE
#else
#define FORCE_INLINE INLINE
#endif

void get_file(const char* file_name, char* data, char*& end_data, size_t size);
void read_int(char*& input, int& n);
void read_char_int(char*& input, int& n);
void eat_non_number(char*& input, char* end);
void eat_non_number_line(char*& input, char* end);
bool peek(const char* input, const char* needle);

extern uint64_t result1;
extern uint64_t result2;