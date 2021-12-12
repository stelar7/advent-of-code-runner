namespace renkon
{
    public static class Utils
    {
        static IEnumerable<string> input;
        public static IEnumerable<string> InputToStringArray(string inputName)
        {
            if (input == null)
            {
                List<string> strs = new List<string>();
                string s;
                while ((s = Console.ReadLine()) != null)
                {
                    strs.Add(s);
                }

                input = strs;
            }

            return input;
        }

        public static IEnumerable<int> StringArrayToIntArray(IEnumerable<string> strArray)
            => strArray.Where(s => !string.IsNullOrEmpty(s)).Select(int.Parse);

        public static IEnumerable<int> GetRangeBetween(int start, int end)
        {
            if (start <= end)
            {
                return Enumerable.Range(start, end - start + 1);
            }

            return Enumerable.Range(end, start - end + 1).Reverse();
        }
    }
}
