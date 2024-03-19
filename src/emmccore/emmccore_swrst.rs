#[doc = "Register `EMMCCORE_SWRST` reader"]
pub type R = crate::R<EmmccoreSwrstSpec>;
#[doc = "Register `EMMCCORE_SWRST` writer"]
pub type W = crate::W<EmmccoreSwrstSpec>;
#[doc = "Software Reset for All\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softwareresetall {
    #[doc = "1: Reset"]
    B1 = 1,
    #[doc = "0: Work This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be reinitialized by the HD."]
    B0 = 0,
}
impl From<Softwareresetall> for bool {
    #[inline(always)]
    fn from(variant: Softwareresetall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTWARERESETALL` reader - Software Reset for All"]
pub type SoftwareresetallR = crate::BitReader<Softwareresetall>;
impl SoftwareresetallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softwareresetall {
        match self.bits {
            true => Softwareresetall::B1,
            false => Softwareresetall::B0,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Softwareresetall::B1
    }
    #[doc = "Work This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be reinitialized by the HD."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Softwareresetall::B0
    }
}
#[doc = "Field `SOFTWARERESETALL` writer - Software Reset for All"]
pub type SoftwareresetallW<'a, REG> = crate::BitWriter<'a, REG, Softwareresetall>;
impl<'a, REG> SoftwareresetallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetall::B1)
    }
    #[doc = "Work This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be reinitialized by the HD."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetall::B0)
    }
}
#[doc = "Only part of command circuit is reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softwareresetcmd {
    #[doc = "1: Reset"]
    B1 = 1,
    #[doc = "0: Work The following registers and bits are cleared by this bit: a. Present State register: Command Inhibit (CMD) b. Normal Interrupt Status register: Command Complete"]
    B0 = 0,
}
impl From<Softwareresetcmd> for bool {
    #[inline(always)]
    fn from(variant: Softwareresetcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTWARERESETCMD` reader - Only part of command circuit is reset."]
pub type SoftwareresetcmdR = crate::BitReader<Softwareresetcmd>;
impl SoftwareresetcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softwareresetcmd {
        match self.bits {
            true => Softwareresetcmd::B1,
            false => Softwareresetcmd::B0,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Softwareresetcmd::B1
    }
    #[doc = "Work The following registers and bits are cleared by this bit: a. Present State register: Command Inhibit (CMD) b. Normal Interrupt Status register: Command Complete"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Softwareresetcmd::B0
    }
}
#[doc = "Field `SOFTWARERESETCMD` writer - Only part of command circuit is reset."]
pub type SoftwareresetcmdW<'a, REG> = crate::BitWriter<'a, REG, Softwareresetcmd>;
impl<'a, REG> SoftwareresetcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetcmd::B1)
    }
    #[doc = "Work The following registers and bits are cleared by this bit: a. Present State register: Command Inhibit (CMD) b. Normal Interrupt Status register: Command Complete"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetcmd::B0)
    }
}
#[doc = "Software Reset for DAT Line.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softwareresetdat {
    #[doc = "1: Reset"]
    B1 = 1,
    #[doc = "0: Work Only part of data circuit is reset. The following registers and bits are cleared by this bit: a. Buffer Data Port Register: Buffer is cleared and Initialized b. Present State register: Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) c. Block Gap Control register: Continue Request Stop At Block Gap Request d. Normal Interrupt Status register: Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
    B0 = 0,
}
impl From<Softwareresetdat> for bool {
    #[inline(always)]
    fn from(variant: Softwareresetdat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTWARERESETDAT` reader - Software Reset for DAT Line."]
pub type SoftwareresetdatR = crate::BitReader<Softwareresetdat>;
impl SoftwareresetdatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softwareresetdat {
        match self.bits {
            true => Softwareresetdat::B1,
            false => Softwareresetdat::B0,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Softwareresetdat::B1
    }
    #[doc = "Work Only part of data circuit is reset. The following registers and bits are cleared by this bit: a. Buffer Data Port Register: Buffer is cleared and Initialized b. Present State register: Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) c. Block Gap Control register: Continue Request Stop At Block Gap Request d. Normal Interrupt Status register: Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Softwareresetdat::B0
    }
}
#[doc = "Field `SOFTWARERESETDAT` writer - Software Reset for DAT Line."]
pub type SoftwareresetdatW<'a, REG> = crate::BitWriter<'a, REG, Softwareresetdat>;
impl<'a, REG> SoftwareresetdatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetdat::B1)
    }
    #[doc = "Work Only part of data circuit is reset. The following registers and bits are cleared by this bit: a. Buffer Data Port Register: Buffer is cleared and Initialized b. Present State register: Buffer read Enable Buffer write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit (DAT) c. Block Gap Control register: Continue Request Stop At Block Gap Request d. Normal Interrupt Status register: Buffer Read Ready Buffer Write Ready Block Gap Event Transfer Complete"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Softwareresetdat::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn softwareresetall(&self) -> SoftwareresetallR {
        SoftwareresetallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only part of command circuit is reset."]
    #[inline(always)]
    pub fn softwareresetcmd(&self) -> SoftwareresetcmdR {
        SoftwareresetcmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line."]
    #[inline(always)]
    pub fn softwareresetdat(&self) -> SoftwareresetdatR {
        SoftwareresetdatR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    #[must_use]
    pub fn softwareresetall(&mut self) -> SoftwareresetallW<EmmccoreSwrstSpec> {
        SoftwareresetallW::new(self, 0)
    }
    #[doc = "Bit 1 - Only part of command circuit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn softwareresetcmd(&mut self) -> SoftwareresetcmdW<EmmccoreSwrstSpec> {
        SoftwareresetcmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line."]
    #[inline(always)]
    #[must_use]
    pub fn softwareresetdat(&mut self) -> SoftwareresetdatW<EmmccoreSwrstSpec> {
        SoftwareresetdatW::new(self, 2)
    }
}
#[doc = "Software reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_swrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_swrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreSwrstSpec;
impl crate::RegisterSpec for EmmccoreSwrstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`emmccore_swrst::R`](R) reader structure"]
impl crate::Readable for EmmccoreSwrstSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_swrst::W`](W) writer structure"]
impl crate::Writable for EmmccoreSwrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_SWRST to value 0"]
impl crate::Resettable for EmmccoreSwrstSpec {
    const RESET_VALUE: u8 = 0;
}
