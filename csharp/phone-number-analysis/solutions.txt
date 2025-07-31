public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        string[] parts = phoneNumber.Split('-');

        string areaCode = parts[0];
        string exchange = parts[1];
        string number = parts[2];

        return (areaCode == "212", exchange == "555", number);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo)
    {
        return phoneNumberInfo.IsFake;
    }
}
