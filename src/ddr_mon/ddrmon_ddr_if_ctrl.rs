#[doc = "Register `DDRMON_DDR_IF_CTRL` reader"]
pub type R = crate::R<DdrmonDdrIfCtrlSpec>;
#[doc = "Register `DDRMON_DDR_IF_CTRL` writer"]
pub type W = crate::W<DdrmonDdrIfCtrlSpec>;
#[doc = "Write or read monitor in channel 0 for command statistics\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0Direction {
    #[doc = "1: read"]
    B1 = 1,
    #[doc = "0: write"]
    B0 = 0,
}
impl From<Ch0Direction> for bool {
    #[inline(always)]
    fn from(variant: Ch0Direction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0_DIRECTION` reader - Write or read monitor in channel 0 for command statistics"]
pub type Ch0DirectionR = crate::BitReader<Ch0Direction>;
impl Ch0DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0Direction {
        match self.bits {
            true => Ch0Direction::B1,
            false => Ch0Direction::B0,
        }
    }
    #[doc = "read"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch0Direction::B1
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch0Direction::B0
    }
}
#[doc = "Field `CH0_DIRECTION` writer - Write or read monitor in channel 0 for command statistics"]
pub type Ch0DirectionW<'a, REG> = crate::BitWriter<'a, REG, Ch0Direction>;
impl<'a, REG> Ch0DirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "read"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Direction::B1)
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Direction::B0)
    }
}
#[doc = "Write or read monitor in channel 1 for command statistics\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1Direction {
    #[doc = "1: read"]
    B1 = 1,
    #[doc = "0: write"]
    B0 = 0,
}
impl From<Ch1Direction> for bool {
    #[inline(always)]
    fn from(variant: Ch1Direction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1_DIRECTION` reader - Write or read monitor in channel 1 for command statistics"]
pub type Ch1DirectionR = crate::BitReader<Ch1Direction>;
impl Ch1DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1Direction {
        match self.bits {
            true => Ch1Direction::B1,
            false => Ch1Direction::B0,
        }
    }
    #[doc = "read"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch1Direction::B1
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch1Direction::B0
    }
}
#[doc = "Field `CH1_DIRECTION` writer - Write or read monitor in channel 1 for command statistics"]
pub type Ch1DirectionW<'a, REG> = crate::BitWriter<'a, REG, Ch1Direction>;
impl<'a, REG> Ch1DirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "read"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Direction::B1)
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Direction::B0)
    }
}
#[doc = "DDR interface and DFI monitor enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfMonEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<IfMonEn> for bool {
    #[inline(always)]
    fn from(variant: IfMonEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF_MON_EN` reader - DDR interface and DFI monitor enable"]
pub type IfMonEnR = crate::BitReader<IfMonEn>;
impl IfMonEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IfMonEn {
        match self.bits {
            true => IfMonEn::B1,
            false => IfMonEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IfMonEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IfMonEn::B0
    }
}
#[doc = "Field `IF_MON_EN` writer - DDR interface and DFI monitor enable"]
pub type IfMonEnW<'a, REG> = crate::BitWriter<'a, REG, IfMonEn>;
impl<'a, REG> IfMonEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IfMonEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IfMonEn::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Write or read monitor in channel 0 for command statistics"]
    #[inline(always)]
    pub fn ch0_direction(&self) -> Ch0DirectionR {
        Ch0DirectionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write or read monitor in channel 1 for command statistics"]
    #[inline(always)]
    pub fn ch1_direction(&self) -> Ch1DirectionR {
        Ch1DirectionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR interface and DFI monitor enable"]
    #[inline(always)]
    pub fn if_mon_en(&self) -> IfMonEnR {
        IfMonEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write or read monitor in channel 0 for command statistics"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_direction(&mut self) -> Ch0DirectionW<DdrmonDdrIfCtrlSpec> {
        Ch0DirectionW::new(self, 0)
    }
    #[doc = "Bit 1 - Write or read monitor in channel 1 for command statistics"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_direction(&mut self) -> Ch1DirectionW<DdrmonDdrIfCtrlSpec> {
        Ch1DirectionW::new(self, 1)
    }
    #[doc = "Bit 2 - DDR interface and DFI monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn if_mon_en(&mut self) -> IfMonEnW<DdrmonDdrIfCtrlSpec> {
        IfMonEnW::new(self, 2)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<DdrmonDdrIfCtrlSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "DDR interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ddr_if_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ddr_if_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonDdrIfCtrlSpec;
impl crate::RegisterSpec for DdrmonDdrIfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ddr_if_ctrl::R`](R) reader structure"]
impl crate::Readable for DdrmonDdrIfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrmon_ddr_if_ctrl::W`](W) writer structure"]
impl crate::Writable for DdrmonDdrIfCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRMON_DDR_IF_CTRL to value 0"]
impl crate::Resettable for DdrmonDdrIfCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
