#[doc = "Register `HIST_WEIGHT_00TO30` reader"]
pub type R = crate::R<HistWeight00to30Spec>;
#[doc = "Register `HIST_WEIGHT_00TO30` writer"]
pub type W = crate::W<HistWeight00to30Spec>;
#[doc = "Field `hist_weight_00` reader - weighting factor for sub-window 00"]
pub type HistWeight00R = crate::FieldReader;
#[doc = "Field `hist_weight_00` writer - weighting factor for sub-window 00"]
pub type HistWeight00W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_10` reader - weighting factor for sub-window 10"]
pub type HistWeight10R = crate::FieldReader;
#[doc = "Field `hist_weight_10` writer - weighting factor for sub-window 10"]
pub type HistWeight10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_20` reader - weighting factor for sub-window 20"]
pub type HistWeight20R = crate::FieldReader;
#[doc = "Field `hist_weight_20` writer - weighting factor for sub-window 20"]
pub type HistWeight20W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_30` reader - weighting factor for sub-window 30"]
pub type HistWeight30R = crate::FieldReader;
#[doc = "Field `hist_weight_30` writer - weighting factor for sub-window 30"]
pub type HistWeight30W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 00"]
    #[inline(always)]
    pub fn hist_weight_00(&self) -> HistWeight00R {
        HistWeight00R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 10"]
    #[inline(always)]
    pub fn hist_weight_10(&self) -> HistWeight10R {
        HistWeight10R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 20"]
    #[inline(always)]
    pub fn hist_weight_20(&self) -> HistWeight20R {
        HistWeight20R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 30"]
    #[inline(always)]
    pub fn hist_weight_30(&self) -> HistWeight30R {
        HistWeight30R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 00"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_00(&mut self) -> HistWeight00W<HistWeight00to30Spec> {
        HistWeight00W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 10"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_10(&mut self) -> HistWeight10W<HistWeight00to30Spec> {
        HistWeight10W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 20"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_20(&mut self) -> HistWeight20W<HistWeight00to30Spec> {
        HistWeight20W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 30"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_30(&mut self) -> HistWeight30W<HistWeight00to30Spec> {
        HistWeight30W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_00to30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_00to30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight00to30Spec;
impl crate::RegisterSpec for HistWeight00to30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_00to30::R`](R) reader structure"]
impl crate::Readable for HistWeight00to30Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_00to30::W`](W) writer structure"]
impl crate::Writable for HistWeight00to30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_00TO30 to value 0x1010_1010"]
impl crate::Resettable for HistWeight00to30Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
