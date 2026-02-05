fn main() {
    {
        let variant: Option<u32> = Some(1);
        dump(variant);
    }
    {
        let variant: Option<u32> = Some(2);
        dump(variant);
    }
    {
        let variant: Option<u32> = Some(3);
        dump(variant);
    }
    {
        let variant: Option<u32> = None;
        dump(variant);
    }
    {
        let variant: Option<u32> = None;
        dump(variant);
    }
    {
        let variant: Option<u32> = Some(255);
        dump(variant);
    }
}

#[inline]
fn dump(variant: Option<u32>) {
    let transmuted: u64 = unsafe { std::mem::transmute(variant) };

    let array = transmuted.to_ne_bytes();

    let (enum_variant, value) = array.split_at(4);

    let enum_variant = u32::from_ne_bytes(into_array::<4, u8>(&enum_variant));
    let value = u32::from_ne_bytes(into_array::<4, u8>(&value));

    println!("Value              : {variant:?}");
    println!("Enum variant       : {enum_variant}");
    println!("Enum variantb      : {enum_variant:0>32b}");
    println!("Enum value         : {value}");
    println!("Enum valueb        : {value:0>32b}");
    println!("Raw bits           : {transmuted:0>64b}");

    println!("---------------------------------------------");
}

fn into_array<'s, const N: usize, T>(slice: &'s [T]) -> [T; N]
where
    [T; N]: TryFrom<&'s [T]>,
    <[T; N] as TryFrom<&'s [T]>>::Error: std::fmt::Debug,
{
    slice.try_into().unwrap()
}
