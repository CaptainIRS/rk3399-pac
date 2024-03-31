#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DFI Timer Count Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerCntEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<TimerCntEn> for bool {
    #[inline(always)]
    fn from(variant: TimerCntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_CNT_EN` reader - DFI Timer Count Enable"]
pub type TimerCntEnR = crate::BitReader<TimerCntEn>;
impl TimerCntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerCntEn {
        match self.bits {
            true => TimerCntEn::B1,
            false => TimerCntEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TimerCntEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TimerCntEn::B0
    }
}
#[doc = "Field `TIMER_CNT_EN` writer - DFI Timer Count Enable"]
pub type TimerCntEnW<'a, REG> = crate::BitWriter<'a, REG, TimerCntEn>;
impl<'a, REG> TimerCntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TimerCntEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TimerCntEn::B0)
    }
}
#[doc = "Software Mode Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoftwareEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<SoftwareEn> for bool {
    #[inline(always)]
    fn from(variant: SoftwareEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTWARE_EN` reader - Software Mode Enable"]
pub type SoftwareEnR = crate::BitReader<SoftwareEn>;
impl SoftwareEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoftwareEn {
        match self.bits {
            true => SoftwareEn::B1,
            false => SoftwareEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SoftwareEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SoftwareEn::B0
    }
}
#[doc = "Field `SOFTWARE_EN` writer - Software Mode Enable"]
pub type SoftwareEnW<'a, REG> = crate::BitWriter<'a, REG, SoftwareEn>;
impl<'a, REG> SoftwareEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SoftwareEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SoftwareEn::B0)
    }
}
#[doc = "LPDDR3 Mode Monitor Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpddr3En {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Lpddr3En> for bool {
    #[inline(always)]
    fn from(variant: Lpddr3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDDR3_EN` reader - LPDDR3 Mode Monitor Enable"]
pub type Lpddr3EnR = crate::BitReader<Lpddr3En>;
impl Lpddr3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpddr3En {
        match self.bits {
            true => Lpddr3En::B1,
            false => Lpddr3En::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpddr3En::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpddr3En::B0
    }
}
#[doc = "Field `LPDDR3_EN` writer - LPDDR3 Mode Monitor Enable"]
pub type Lpddr3EnW<'a, REG> = crate::BitWriter<'a, REG, Lpddr3En>;
impl<'a, REG> Lpddr3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpddr3En::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpddr3En::B0)
    }
}
#[doc = "Hardware Mode Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HardwareEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<HardwareEn> for bool {
    #[inline(always)]
    fn from(variant: HardwareEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HARDWARE_EN` reader - Hardware Mode Enable"]
pub type HardwareEnR = crate::BitReader<HardwareEn>;
impl HardwareEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HardwareEn {
        match self.bits {
            true => HardwareEn::B1,
            false => HardwareEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HardwareEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HardwareEn::B0
    }
}
#[doc = "Field `HARDWARE_EN` writer - Hardware Mode Enable"]
pub type HardwareEnW<'a, REG> = crate::BitWriter<'a, REG, HardwareEn>;
impl<'a, REG> HardwareEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HardwareEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HardwareEn::B0)
    }
}
#[doc = "LPDDR4 Mode Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpddr4En {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Lpddr4En> for bool {
    #[inline(always)]
    fn from(variant: Lpddr4En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDDR4_EN` reader - LPDDR4 Mode Enable"]
pub type Lpddr4EnR = crate::BitReader<Lpddr4En>;
impl Lpddr4EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpddr4En {
        match self.bits {
            true => Lpddr4En::B1,
            false => Lpddr4En::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpddr4En::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpddr4En::B0
    }
}
#[doc = "Field `LPDDR4_EN` writer - LPDDR4 Mode Enable"]
pub type Lpddr4EnW<'a, REG> = crate::BitWriter<'a, REG, Lpddr4En>;
impl<'a, REG> Lpddr4EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpddr4En::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpddr4En::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by softwar .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by softwar .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - DFI Timer Count Enable"]
    #[inline(always)]
    pub fn timer_cnt_en(&self) -> TimerCntEnR {
        TimerCntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Mode Enable"]
    #[inline(always)]
    pub fn software_en(&self) -> SoftwareEnR {
        SoftwareEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPDDR3 Mode Monitor Enable"]
    #[inline(always)]
    pub fn lpddr3_en(&self) -> Lpddr3EnR {
        Lpddr3EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware Mode Enable"]
    #[inline(always)]
    pub fn hardware_en(&self) -> HardwareEnR {
        HardwareEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPDDR4 Mode Enable"]
    #[inline(always)]
    pub fn lpddr4_en(&self) -> Lpddr4EnR {
        Lpddr4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by softwar .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - DFI Timer Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_cnt_en(&mut self) -> TimerCntEnW<CtrlSpec> {
        TimerCntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn software_en(&mut self) -> SoftwareEnW<CtrlSpec> {
        SoftwareEnW::new(self, 1)
    }
    #[doc = "Bit 2 - LPDDR3 Mode Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3_en(&mut self) -> Lpddr3EnW<CtrlSpec> {
        Lpddr3EnW::new(self, 2)
    }
    #[doc = "Bit 3 - Hardware Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_en(&mut self) -> HardwareEnW<CtrlSpec> {
        HardwareEnW::new(self, 3)
    }
    #[doc = "Bit 4 - LPDDR4 Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr4_en(&mut self) -> Lpddr4EnW<CtrlSpec> {
        Lpddr4EnW::new(self, 4)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by softwar .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<CtrlSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "DDR Monitor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
