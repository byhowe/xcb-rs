use super::xcb::xcb_image_order_t;

#[inline(always)]
pub unsafe extern "C" fn xcb_mask(n: u32) -> u32
{
  if n == 32 {
    !0
  } else {
    (1 << n) - 1
  }
}

#[inline(always)]
pub unsafe extern "C" fn xcb_popcount(mut x: u32) -> u32
{
  let m1: u32 = 0x55555555;
  let m2: u32 = 0x33333333;
  let m4: u32 = 0x0f0f0f0f;
  x -= (x >> 1) & m1;
  x = (x & m2) + ((x >> 2) & m2);
  x = (x + (x >> 4)) & m4;
  x += x >> 8;
  (x + (x >> 16)) & 0x3f
}

#[inline(always)]
pub unsafe extern "C" fn xcb_roundup_2(
  base: u32,
  pad: u32,
) -> u32
{
  (base + pad - 1) & (u32::MAX - pad + 1)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_rounddown_2(
  base: u32,
  pad: u32,
) -> u32
{
  base & (u32::MAX - pad + 1)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_roundup(
  base: u32,
  pad: u32,
) -> u32
{
  let b: u32 = base + pad - 1;
  if ((pad - 1) & pad) == 0 {
    b & (u32::MAX - pad + 1)
  } else {
    b - b % pad
  }
}

#[inline(always)]
pub unsafe extern "C" fn xcb_rounddown(
  base: u32,
  pad: u32,
) -> u32
{
  if ((pad - 1) & pad) == 0 {
    base & (u32::MAX - pad + 1)
  } else {
    base - base % pad
  }
}

#[inline(always)]
pub unsafe extern "C" fn xcb_bit_reverse(
  mut x: u32,
  n: u8,
) -> u32
{
  let m1: u32 = 0x00ff00ff;
  let m2: u32 = 0x0f0f0f0f;
  let m3: u32 = 0x33333333;
  let m4: u32 = 0x55555555;
  x = (x << 16) | (x >> 16);
  x = ((x & m1) << 8) | ((x >> 8) & m1);
  x = ((x & m2) << 4) | ((x >> 4) & m2);
  x = ((x & m3) << 2) | ((x >> 2) & m3);
  x = ((x & m4) << 1) | ((x >> 1) & m4);
  x >>= 32 - n;
  x
}

#[inline(always)]
pub unsafe extern "C" fn xcb_host_byte_order() -> xcb_image_order_t
{
  let endian_test: *const u32 = &0x01020304;

  match *(endian_test as *const libc::c_char) {
    0x01 => xcb_image_order_t::XCB_IMAGE_ORDER_MSB_FIRST,
    0x04 => xcb_image_order_t::XCB_IMAGE_ORDER_LSB_FIRST,
    _ => panic!("unexpected: bitops::xcb_host_byte_order"),
  }
}
