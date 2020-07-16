#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    #[doc = "1: Flash programming activated"]
    PROGRAM = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PG`"]
pub type PG_R = crate::R<bool, PG_A>;
impl PG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PG_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PG_A::PROGRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROGRAM`"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG_A::PROGRAM
    }
}
#[doc = "Write proxy for field `PG`"]
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn program(self) -> &'a mut W {
        self.variant(PG_A::PROGRAM)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Sector Erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SER_A {
    #[doc = "1: Erase activated for selected sector"]
    SECTORERASE = 1,
}
impl From<SER_A> for bool {
    #[inline(always)]
    fn from(variant: SER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SER`"]
pub type SER_R = crate::R<bool, SER_A>;
impl SER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SER_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SER_A::SECTORERASE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SECTORERASE`"]
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SER_A::SECTORERASE
    }
}
#[doc = "Write proxy for field `SER`"]
pub struct SER_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Erase activated for selected sector"]
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut W {
        self.variant(SER_A::SECTORERASE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Mass Erase of sectors 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER_A {
    #[doc = "1: Erase activated for all user sectors"]
    MASSERASE = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MER`"]
pub type MER_R = crate::R<bool, MER_A>;
impl MER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MER_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MER_A::MASSERASE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASSERASE`"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER_A::MASSERASE
    }
}
#[doc = "Write proxy for field `MER`"]
pub struct MER_W<'a> {
    w: &'a mut W,
}
impl<'a> MER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MASSERASE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Sector number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SNB_A {
    #[doc = "0: Sector 0"]
    SECTOR0 = 0,
    #[doc = "1: Sector 1"]
    SECTOR1 = 1,
    #[doc = "2: Sector 2"]
    SECTOR2 = 2,
    #[doc = "3: Sector 3"]
    SECTOR3 = 3,
    #[doc = "4: Sector 4"]
    SECTOR4 = 4,
    #[doc = "5: Sector 5"]
    SECTOR5 = 5,
    #[doc = "6: Sector 6"]
    SECTOR6 = 6,
    #[doc = "7: Sector 7"]
    SECTOR7 = 7,
    #[doc = "12: User-specific sector"]
    USERSPECIFIC = 12,
    #[doc = "13: User configuration sector"]
    USERCONFIG = 13,
}
impl From<SNB_A> for u8 {
    #[inline(always)]
    fn from(variant: SNB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SNB`"]
pub type SNB_R = crate::R<u8, SNB_A>;
impl SNB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SNB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SNB_A::SECTOR0),
            1 => Val(SNB_A::SECTOR1),
            2 => Val(SNB_A::SECTOR2),
            3 => Val(SNB_A::SECTOR3),
            4 => Val(SNB_A::SECTOR4),
            5 => Val(SNB_A::SECTOR5),
            6 => Val(SNB_A::SECTOR6),
            7 => Val(SNB_A::SECTOR7),
            12 => Val(SNB_A::USERSPECIFIC),
            13 => Val(SNB_A::USERCONFIG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SECTOR0`"]
    #[inline(always)]
    pub fn is_sector0(&self) -> bool {
        *self == SNB_A::SECTOR0
    }
    #[doc = "Checks if the value of the field is `SECTOR1`"]
    #[inline(always)]
    pub fn is_sector1(&self) -> bool {
        *self == SNB_A::SECTOR1
    }
    #[doc = "Checks if the value of the field is `SECTOR2`"]
    #[inline(always)]
    pub fn is_sector2(&self) -> bool {
        *self == SNB_A::SECTOR2
    }
    #[doc = "Checks if the value of the field is `SECTOR3`"]
    #[inline(always)]
    pub fn is_sector3(&self) -> bool {
        *self == SNB_A::SECTOR3
    }
    #[doc = "Checks if the value of the field is `SECTOR4`"]
    #[inline(always)]
    pub fn is_sector4(&self) -> bool {
        *self == SNB_A::SECTOR4
    }
    #[doc = "Checks if the value of the field is `SECTOR5`"]
    #[inline(always)]
    pub fn is_sector5(&self) -> bool {
        *self == SNB_A::SECTOR5
    }
    #[doc = "Checks if the value of the field is `SECTOR6`"]
    #[inline(always)]
    pub fn is_sector6(&self) -> bool {
        *self == SNB_A::SECTOR6
    }
    #[doc = "Checks if the value of the field is `SECTOR7`"]
    #[inline(always)]
    pub fn is_sector7(&self) -> bool {
        *self == SNB_A::SECTOR7
    }
    #[doc = "Checks if the value of the field is `USERSPECIFIC`"]
    #[inline(always)]
    pub fn is_user_specific(&self) -> bool {
        *self == SNB_A::USERSPECIFIC
    }
    #[doc = "Checks if the value of the field is `USERCONFIG`"]
    #[inline(always)]
    pub fn is_user_config(&self) -> bool {
        *self == SNB_A::USERCONFIG
    }
}
#[doc = "Write proxy for field `SNB`"]
pub struct SNB_W<'a> {
    w: &'a mut W,
}
impl<'a> SNB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sector 0"]
    #[inline(always)]
    pub fn sector0(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR0)
    }
    #[doc = "Sector 1"]
    #[inline(always)]
    pub fn sector1(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR1)
    }
    #[doc = "Sector 2"]
    #[inline(always)]
    pub fn sector2(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR2)
    }
    #[doc = "Sector 3"]
    #[inline(always)]
    pub fn sector3(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR3)
    }
    #[doc = "Sector 4"]
    #[inline(always)]
    pub fn sector4(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR4)
    }
    #[doc = "Sector 5"]
    #[inline(always)]
    pub fn sector5(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR5)
    }
    #[doc = "Sector 6"]
    #[inline(always)]
    pub fn sector6(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR6)
    }
    #[doc = "Sector 7"]
    #[inline(always)]
    pub fn sector7(self) -> &'a mut W {
        self.variant(SNB_A::SECTOR7)
    }
    #[doc = "User-specific sector"]
    #[inline(always)]
    pub fn user_specific(self) -> &'a mut W {
        self.variant(SNB_A::USERSPECIFIC)
    }
    #[doc = "User configuration sector"]
    #[inline(always)]
    pub fn user_config(self) -> &'a mut W {
        self.variant(SNB_A::USERCONFIG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Program size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: Programming parallelism x8"]
    BITS8 = 0,
    #[doc = "1: Programming parallelism x16"]
    BITS16 = 1,
    #[doc = "2: Programming parallelism x32"]
    BITS32 = 2,
    #[doc = "3: Programming parallelism x64"]
    BITS64 = 3,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSIZE`"]
pub type PSIZE_R = crate::R<u8, PSIZE_A>;
impl PSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::BITS8,
            1 => PSIZE_A::BITS16,
            2 => PSIZE_A::BITS32,
            3 => PSIZE_A::BITS64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE_A::BITS32
    }
    #[doc = "Checks if the value of the field is `BITS64`"]
    #[inline(always)]
    pub fn is_bits64(&self) -> bool {
        *self == PSIZE_A::BITS64
    }
}
#[doc = "Write proxy for field `PSIZE`"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Programming parallelism x8"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS8)
    }
    #[doc = "Programming parallelism x16"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS16)
    }
    #[doc = "Programming parallelism x32"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS32)
    }
    #[doc = "Programming parallelism x64"]
    #[inline(always)]
    pub fn bits64(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    #[doc = "1: Trigger an erase operation"]
    START = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STRT`"]
pub type STRT_R = crate::R<bool, STRT_A>;
impl STRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STRT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STRT_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT_A::START
    }
}
#[doc = "Write proxy for field `STRT`"]
pub struct STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRT_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "End of operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIE_A {
    #[doc = "0: End of operation interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: End of operation interrupt enabled"]
    ENABLED = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOPIE`"]
pub type EOPIE_R = crate::R<bool, EOPIE_A>;
impl EOPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::DISABLED,
            true => EOPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `EOPIE`"]
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::DISABLED)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt generation disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt generation enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: FLASH_CR register is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: FLASH_CR register is locked"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_AW {
    #[doc = "1: Lock the FLASH_CR register"]
    LOCK = 1,
}
impl From<LOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lock the FLASH_CR register"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_AW::LOCK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - Sector number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W {
        SER_W { w: self }
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W { w: self }
    }
    #[doc = "Bits 3:6 - Sector number"]
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W {
        SNB_W { w: self }
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
