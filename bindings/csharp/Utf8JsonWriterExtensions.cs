using System.Text;
using System.Text.Json;

namespace NeatProto;

public static class Utf8JsonWriterExtensions
{
    public static void WriteValue(this Utf8JsonWriter writer, byte value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, ushort value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, uint value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, ulong value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, UInt128 value)
        => writer.WriteRawValue(Encoding.UTF8.GetBytes(value.ToString()), true);

    public static void WriteValue(this Utf8JsonWriter writer, sbyte value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, short value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, int value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, long value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, Int128 value)
        => writer.WriteRawValue(Encoding.UTF8.GetBytes(value.ToString()), true);

    public static void WriteValue(this Utf8JsonWriter writer, float value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, double value)
        => writer.WriteNumberValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, bool value)
        => writer.WriteBooleanValue(value);

    public static void WriteValue(this Utf8JsonWriter writer, string value)
        => writer.WriteStringValue(value);
}