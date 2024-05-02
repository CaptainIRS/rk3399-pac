#[doc = "Register `HIST_WEIGHT_22TO03` reader"]
pub type R = crate::R<HistWeight22to03Spec>;
#[doc = "Register `HIST_WEIGHT_22TO03` writer"]
pub type W = crate::W<HistWeight22to03Spec>;
#[doc = "Field `hist_weight_22` reader - weighting factor for sub-window 22"]
pub type HistWeight22R = crate::FieldReader;
#[doc = "Field `hist_weight_22` writer - weighting factor for sub-window 22"]
pub type HistWeight22W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_32` reader - weighting factor for sub-window 32"]
pub type HistWeight32R = crate::FieldReader;
#[doc = "Field `hist_weight_32` writer - weighting factor for sub-window 32"]
pub type HistWeight32W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_42` reader - weighting factor for sub-window 42\n\n"]
pub type HistWeight42R = crate::FieldReader;
#[doc = "Field `hist_weight_42` writer - weighting factor for sub-window 42\n\n"]
pub type HistWeight42W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_03` reader - weighting factor for sub-window 03"]
pub type HistWeight03R = crate::FieldReader;
#[doc = "Field `hist_weight_03` writer - weighting factor for sub-window 03"]
pub type HistWeight03W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 22"]
    #[inline(always)]
    pub fn hist_weight_22(&self) -> HistWeight22R {
        HistWeight22R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 32"]
    #[inline(always)]
    pub fn hist_weight_32(&self) -> HistWeight32R {
        HistWeight32R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 42\n\n"]
    #[inline(always)]
    pub fn hist_weight_42(&self) -> HistWeight42R {
        HistWeight42R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 03"]
    #[inline(always)]
    pub fn hist_weight_03(&self) -> HistWeight03R {
        HistWeight03R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 22"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_22(&mut self) -> HistWeight22W<HistWeight22to03Spec> {
        HistWeight22W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 32"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_32(&mut self) -> HistWeight32W<HistWeight22to03Spec> {
        HistWeight32W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 42\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_42(&mut self) -> HistWeight42W<HistWeight22to03Spec> {
        HistWeight42W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 03"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_03(&mut self) -> HistWeight03W<HistWeight22to03Spec> {
        HistWeight03W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_22to03::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_22to03::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight22to03Spec;
impl crate::RegisterSpec for HistWeight22to03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_22to03::R`](R) reader structure"]
impl crate::Readable for HistWeight22to03Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_22to03::W`](W) writer structure"]
impl crate::Writable for HistWeight22to03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_22TO03 to value 0x1010_1010"]
impl crate::Resettable for HistWeight22to03Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
