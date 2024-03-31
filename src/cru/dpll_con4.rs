#[doc = "Register `DPLL_CON4` reader"]
pub type R = crate::R<DpllCon4Spec>;
#[doc = "Register `DPLL_CON4` writer"]
pub type W = crate::W<DpllCon4Spec>;
#[doc = "Bypass SSMOD by integration\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsmodBp {
    #[doc = "0: no bypass"]
    B0 = 0,
    #[doc = "1: bypass"]
    B1 = 1,
}
impl From<SsmodBp> for bool {
    #[inline(always)]
    fn from(variant: SsmodBp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSMOD_BP` reader - Bypass SSMOD by integration"]
pub type SsmodBpR = crate::BitReader<SsmodBp>;
impl SsmodBpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsmodBp {
        match self.bits {
            false => SsmodBp::B0,
            true => SsmodBp::B1,
        }
    }
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SsmodBp::B0
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SsmodBp::B1
    }
}
#[doc = "Field `SSMOD_BP` writer - Bypass SSMOD by integration"]
pub type SsmodBpW<'a, REG> = crate::BitWriter<'a, REG, SsmodBp>;
impl<'a, REG> SsmodBpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodBp::B0)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodBp::B1)
    }
}
#[doc = "Bypass SSMOD by module\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsmodDisableSscg {
    #[doc = "0: no bypass"]
    B0 = 0,
    #[doc = "1: bypass"]
    B1 = 1,
}
impl From<SsmodDisableSscg> for bool {
    #[inline(always)]
    fn from(variant: SsmodDisableSscg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSMOD_DISABLE_SSCG` reader - Bypass SSMOD by module"]
pub type SsmodDisableSscgR = crate::BitReader<SsmodDisableSscg>;
impl SsmodDisableSscgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsmodDisableSscg {
        match self.bits {
            false => SsmodDisableSscg::B0,
            true => SsmodDisableSscg::B1,
        }
    }
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SsmodDisableSscg::B0
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SsmodDisableSscg::B1
    }
}
#[doc = "Field `SSMOD_DISABLE_SSCG` writer - Bypass SSMOD by module"]
pub type SsmodDisableSscgW<'a, REG> = crate::BitWriter<'a, REG, SsmodDisableSscg>;
impl<'a, REG> SsmodDisableSscgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodDisableSscg::B0)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodDisableSscg::B1)
    }
}
#[doc = "Reset modulator state\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsmodReset {
    #[doc = "0: no reset"]
    B0 = 0,
    #[doc = "1: reset"]
    B1 = 1,
}
impl From<SsmodReset> for bool {
    #[inline(always)]
    fn from(variant: SsmodReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSMOD_RESET` reader - Reset modulator state"]
pub type SsmodResetR = crate::BitReader<SsmodReset>;
impl SsmodResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsmodReset {
        match self.bits {
            false => SsmodReset::B0,
            true => SsmodReset::B1,
        }
    }
    #[doc = "no reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SsmodReset::B0
    }
    #[doc = "reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SsmodReset::B1
    }
}
#[doc = "Field `SSMOD_RESET` writer - Reset modulator state"]
pub type SsmodResetW<'a, REG> = crate::BitWriter<'a, REG, SsmodReset>;
impl<'a, REG> SsmodResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodReset::B0)
    }
    #[doc = "reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodReset::B1)
    }
}
#[doc = "Selects center spread or downs pread\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsmodDownspread {
    #[doc = "0: center spread"]
    B0 = 0,
    #[doc = "1: down spread"]
    B1 = 1,
}
impl From<SsmodDownspread> for bool {
    #[inline(always)]
    fn from(variant: SsmodDownspread) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSMOD_DOWNSPREAD` reader - Selects center spread or downs pread"]
pub type SsmodDownspreadR = crate::BitReader<SsmodDownspread>;
impl SsmodDownspreadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsmodDownspread {
        match self.bits {
            false => SsmodDownspread::B0,
            true => SsmodDownspread::B1,
        }
    }
    #[doc = "center spread"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SsmodDownspread::B0
    }
    #[doc = "down spread"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SsmodDownspread::B1
    }
}
#[doc = "Field `SSMOD_DOWNSPREAD` writer - Selects center spread or downs pread"]
pub type SsmodDownspreadW<'a, REG> = crate::BitWriter<'a, REG, SsmodDownspread>;
impl<'a, REG> SsmodDownspreadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "center spread"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodDownspread::B0)
    }
    #[doc = "down spread"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodDownspread::B1)
    }
}
#[doc = "Field `SSMOD_DIVVAL` reader - Divider required to set the modulation frequency\n\nDivider required to set the modulation frequency"]
pub type SsmodDivvalR = crate::FieldReader;
#[doc = "Field `SSMOD_DIVVAL` writer - Divider required to set the modulation frequency\n\nDivider required to set the modulation frequency"]
pub type SsmodDivvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SSMOD_SPREAD` reader - spread amplitude\n\n% = 0.1 * SPREAD\\[4:0\\]"]
pub type SsmodSpreadR = crate::FieldReader;
#[doc = "Field `SSMOD_SPREAD` writer - spread amplitude\n\n% = 0.1 * SPREAD\\[4:0\\]"]
pub type SsmodSpreadW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Bypass SSMOD by integration"]
    #[inline(always)]
    pub fn ssmod_bp(&self) -> SsmodBpR {
        SsmodBpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bypass SSMOD by module"]
    #[inline(always)]
    pub fn ssmod_disable_sscg(&self) -> SsmodDisableSscgR {
        SsmodDisableSscgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset modulator state"]
    #[inline(always)]
    pub fn ssmod_reset(&self) -> SsmodResetR {
        SsmodResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects center spread or downs pread"]
    #[inline(always)]
    pub fn ssmod_downspread(&self) -> SsmodDownspreadR {
        SsmodDownspreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Divider required to set the modulation frequency\n\nDivider required to set the modulation frequency"]
    #[inline(always)]
    pub fn ssmod_divval(&self) -> SsmodDivvalR {
        SsmodDivvalR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - spread amplitude\n\n% = 0.1 * SPREAD\\[4:0\\]"]
    #[inline(always)]
    pub fn ssmod_spread(&self) -> SsmodSpreadR {
        SsmodSpreadR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass SSMOD by integration"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_bp(&mut self) -> SsmodBpW<DpllCon4Spec> {
        SsmodBpW::new(self, 0)
    }
    #[doc = "Bit 1 - Bypass SSMOD by module"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_disable_sscg(&mut self) -> SsmodDisableSscgW<DpllCon4Spec> {
        SsmodDisableSscgW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset modulator state"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_reset(&mut self) -> SsmodResetW<DpllCon4Spec> {
        SsmodResetW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects center spread or downs pread"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_downspread(&mut self) -> SsmodDownspreadW<DpllCon4Spec> {
        SsmodDownspreadW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Divider required to set the modulation frequency\n\nDivider required to set the modulation frequency"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_divval(&mut self) -> SsmodDivvalW<DpllCon4Spec> {
        SsmodDivvalW::new(self, 4)
    }
    #[doc = "Bits 8:12 - spread amplitude\n\n% = 0.1 * SPREAD\\[4:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_spread(&mut self) -> SsmodSpreadW<DpllCon4Spec> {
        SsmodSpreadW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<DpllCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "DPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllCon4Spec;
impl crate::RegisterSpec for DpllCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpll_con4::R`](R) reader structure"]
impl crate::Readable for DpllCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`dpll_con4::W`](W) writer structure"]
impl crate::Writable for DpllCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPLL_CON4 to value 0x07"]
impl crate::Resettable for DpllCon4Spec {
    const RESET_VALUE: u32 = 0x07;
}
