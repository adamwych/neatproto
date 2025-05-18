using System.ComponentModel.DataAnnotations;

namespace NeatProto;

public class TaggedUnion<TKind>
{
    [Required]
    public required TKind Kind { get; init; }
}

public class TaggedUnion<TKind, TValue> : TaggedUnion<TKind>
    where TValue : ITaggedUnionCase<TKind>
{
    // Disable nullability warnings, because unit enum variants do not include this
    // property in their JSON representation, and there's no way to nicely represent that in C#.
#nullable disable
    public TValue Value { get; init; }
#nullable enable
}

public interface ITaggedUnionCase<out TKind>
{
    public static abstract TKind Kind { get; }
}