using System.Text;
using System.Text.Json;

namespace NeatProto;

public static class Utf8JsonReaderExtensions
{
    public static void Get(this Utf8JsonReader reader, out byte value)
        => value = reader.GetByte();

    public static void Get(this Utf8JsonReader reader, out ushort value)
        => value = reader.GetUInt16();

    public static void Get(this Utf8JsonReader reader, out uint value)
        => value = reader.GetUInt32();

    public static void Get(this Utf8JsonReader reader, out ulong value)
        => value = reader.GetUInt64();

    public static void Get(this Utf8JsonReader reader, out UInt128 value)
        => value = UInt128.Parse(Encoding.UTF8.GetString(reader.ValueSpan));

    public static void Get(this Utf8JsonReader reader, out sbyte value)
        => value = reader.GetSByte();

    public static void Get(this Utf8JsonReader reader, out short value)
        => value = reader.GetInt16();

    public static void Get(this Utf8JsonReader reader, out int value)
        => value = reader.GetInt32();

    public static void Get(this Utf8JsonReader reader, out long value)
        => value = reader.GetInt64();

    public static void Get(this Utf8JsonReader reader, out Int128 value)
        => value = Int128.Parse(Encoding.UTF8.GetString(reader.ValueSpan));

    public static void Get(this Utf8JsonReader reader, out float value)
        => value = reader.GetSingle();
    
    public static void Get(this Utf8JsonReader reader, out double value)
        => value = reader.GetDouble();
    
    public static void Get(this Utf8JsonReader reader, out bool value)
        => value = reader.GetBoolean();
    
    public static void Get(this Utf8JsonReader reader, out string value)
        => value = reader.GetString()!;
}