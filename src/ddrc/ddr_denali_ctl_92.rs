#[doc = "Register `DDR_DENALI_CTL_92` reader"]
pub type R = crate::R<DdrDenaliCtl92Spec>;
#[doc = "Register `DDR_DENALI_CTL_92` writer"]
pub type W = crate::W<DdrDenaliCtl92Spec>;
#[doc = "Field `LOWPOWER_REFRESH_ENABLE` reader - Enable refreshes while in low power mode. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
pub type LowpowerRefreshEnableR = crate::FieldReader;
#[doc = "Field `LOWPOWER_REFRESH_ENABLE` writer - Enable refreshes while in low power mode. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
pub type LowpowerRefreshEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKSRE_F0` reader - Clock hold delay on self-refresh entry for frequency copy 0."]
pub type CksreF0R = crate::FieldReader;
#[doc = "Field `CKSRE_F0` writer - Clock hold delay on self-refresh entry for frequency copy 0."]
pub type CksreF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRX_F0` reader - Clock stable delay on self-refresh exit for frequency copy 0."]
pub type CksrxF0R = crate::FieldReader;
#[doc = "Field `CKSRX_F0` writer - Clock stable delay on self-refresh exit for frequency copy 0."]
pub type CksrxF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRE_F1` reader - Clock hold delay on self-refresh entry for frequency copy 1."]
pub type CksreF1R = crate::FieldReader;
#[doc = "Field `CKSRE_F1` writer - Clock hold delay on self-refresh entry for frequency copy 1."]
pub type CksreF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Enable refreshes while in low power mode. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    pub fn lowpower_refresh_enable(&self) -> LowpowerRefreshEnableR {
        LowpowerRefreshEnableR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - Clock hold delay on self-refresh entry for frequency copy 0."]
    #[inline(always)]
    pub fn cksre_f0(&self) -> CksreF0R {
        CksreF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock stable delay on self-refresh exit for frequency copy 0."]
    #[inline(always)]
    pub fn cksrx_f0(&self) -> CksrxF0R {
        CksrxF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock hold delay on self-refresh entry for frequency copy 1."]
    #[inline(always)]
    pub fn cksre_f1(&self) -> CksreF1R {
        CksreF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable refreshes while in low power mode. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn lowpower_refresh_enable(&mut self) -> LowpowerRefreshEnableW<DdrDenaliCtl92Spec> {
        LowpowerRefreshEnableW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock hold delay on self-refresh entry for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f0(&mut self) -> CksreF0W<DdrDenaliCtl92Spec> {
        CksreF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock stable delay on self-refresh exit for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f0(&mut self) -> CksrxF0W<DdrDenaliCtl92Spec> {
        CksrxF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock hold delay on self-refresh entry for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f1(&mut self) -> CksreF1W<DdrDenaliCtl92Spec> {
        CksreF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl92Spec;
impl crate::RegisterSpec for DdrDenaliCtl92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_92::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl92Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_92::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_92 to value 0"]
impl crate::Resettable for DdrDenaliCtl92Spec {
    const RESET_VALUE: u32 = 0;
}
