#[doc = "Register `AUX_CH_DEFER_CTL` reader"]
pub type R = crate::R<AuxChDeferCtlSpec>;
#[doc = "Register `AUX_CH_DEFER_CTL` writer"]
pub type W = crate::W<AuxChDeferCtlSpec>;
#[doc = "Field `DEFER_COUNT` reader - The count is defined to limit the max count AUX CH receive DEFER command When DEFER_CTRL_EN is 1 and AUX CH received (DEFER_COUNT * 64) DEFER command, the AUX CH will terminate the transaction"]
pub type DeferCountR = crate::FieldReader;
#[doc = "Field `DEFER_COUNT` writer - The count is defined to limit the max count AUX CH receive DEFER command When DEFER_CTRL_EN is 1 and AUX CH received (DEFER_COUNT * 64) DEFER command, the AUX CH will terminate the transaction"]
pub type DeferCountW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "AUX CH received DEFER command count control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeferCtrlEn {
    #[doc = "1: The count that AUX CH receive DEFER command is unlimited"]
    B1 = 1,
    #[doc = "0: The count that AUX CH receive DEFER command is unlimited"]
    B0 = 0,
}
impl From<DeferCtrlEn> for bool {
    #[inline(always)]
    fn from(variant: DeferCtrlEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFER_CTRL_EN` reader - AUX CH received DEFER command count control enable"]
pub type DeferCtrlEnR = crate::BitReader<DeferCtrlEn>;
impl DeferCtrlEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeferCtrlEn {
        match self.bits {
            true => DeferCtrlEn::B1,
            false => DeferCtrlEn::B0,
        }
    }
    #[doc = "The count that AUX CH receive DEFER command is unlimited"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DeferCtrlEn::B1
    }
    #[doc = "The count that AUX CH receive DEFER command is unlimited"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DeferCtrlEn::B0
    }
}
#[doc = "Field `DEFER_CTRL_EN` writer - AUX CH received DEFER command count control enable"]
pub type DeferCtrlEnW<'a, REG> = crate::BitWriter1C<'a, REG, DeferCtrlEn>;
impl<'a, REG> DeferCtrlEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The count that AUX CH receive DEFER command is unlimited"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DeferCtrlEn::B1)
    }
    #[doc = "The count that AUX CH receive DEFER command is unlimited"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DeferCtrlEn::B0)
    }
}
impl R {
    #[doc = "Bits 0:6 - The count is defined to limit the max count AUX CH receive DEFER command When DEFER_CTRL_EN is 1 and AUX CH received (DEFER_COUNT * 64) DEFER command, the AUX CH will terminate the transaction"]
    #[inline(always)]
    pub fn defer_count(&self) -> DeferCountR {
        DeferCountR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - AUX CH received DEFER command count control enable"]
    #[inline(always)]
    pub fn defer_ctrl_en(&self) -> DeferCtrlEnR {
        DeferCtrlEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The count is defined to limit the max count AUX CH receive DEFER command When DEFER_CTRL_EN is 1 and AUX CH received (DEFER_COUNT * 64) DEFER command, the AUX CH will terminate the transaction"]
    #[inline(always)]
    #[must_use]
    pub fn defer_count(&mut self) -> DeferCountW<AuxChDeferCtlSpec> {
        DeferCountW::new(self, 0)
    }
    #[doc = "Bit 7 - AUX CH received DEFER command count control enable"]
    #[inline(always)]
    #[must_use]
    pub fn defer_ctrl_en(&mut self) -> DeferCtrlEnW<AuxChDeferCtlSpec> {
        DeferCtrlEnW::new(self, 7)
    }
}
#[doc = "DP AUX CH DEFER Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_defer_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_defer_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxChDeferCtlSpec;
impl crate::RegisterSpec for AuxChDeferCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_ch_defer_ctl::R`](R) reader structure"]
impl crate::Readable for AuxChDeferCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`aux_ch_defer_ctl::W`](W) writer structure"]
impl crate::Writable for AuxChDeferCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets AUX_CH_DEFER_CTL to value 0x7f"]
impl crate::Resettable for AuxChDeferCtlSpec {
    const RESET_VALUE: u32 = 0x7f;
}
