#[doc = "Register `HIST_WEIGHT_40TO21` reader"]
pub type R = crate::R<HistWeight40to21Spec>;
#[doc = "Register `HIST_WEIGHT_40TO21` writer"]
pub type W = crate::W<HistWeight40to21Spec>;
#[doc = "Field `hist_weight_40` reader - weighting factor for sub-window 40"]
pub type HistWeight40R = crate::FieldReader;
#[doc = "Field `hist_weight_40` writer - weighting factor for sub-window 40"]
pub type HistWeight40W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_01` reader - weighting factor for sub-window 01"]
pub type HistWeight01R = crate::FieldReader;
#[doc = "Field `hist_weight_01` writer - weighting factor for sub-window 01"]
pub type HistWeight01W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_11` reader - weighting factor for sub-window 11"]
pub type HistWeight11R = crate::FieldReader;
#[doc = "Field `hist_weight_11` writer - weighting factor for sub-window 11"]
pub type HistWeight11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_21` reader - weighting factor for sub-window 21\n\n"]
pub type HistWeight21R = crate::FieldReader;
#[doc = "Field `hist_weight_21` writer - weighting factor for sub-window 21\n\n"]
pub type HistWeight21W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 40"]
    #[inline(always)]
    pub fn hist_weight_40(&self) -> HistWeight40R {
        HistWeight40R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 01"]
    #[inline(always)]
    pub fn hist_weight_01(&self) -> HistWeight01R {
        HistWeight01R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 11"]
    #[inline(always)]
    pub fn hist_weight_11(&self) -> HistWeight11R {
        HistWeight11R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 21\n\n"]
    #[inline(always)]
    pub fn hist_weight_21(&self) -> HistWeight21R {
        HistWeight21R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 40"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_40(&mut self) -> HistWeight40W<HistWeight40to21Spec> {
        HistWeight40W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 01"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_01(&mut self) -> HistWeight01W<HistWeight40to21Spec> {
        HistWeight01W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 11"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_11(&mut self) -> HistWeight11W<HistWeight40to21Spec> {
        HistWeight11W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 21\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_21(&mut self) -> HistWeight21W<HistWeight40to21Spec> {
        HistWeight21W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\n\n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\n\n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_40to21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_40to21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight40to21Spec;
impl crate::RegisterSpec for HistWeight40to21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_40to21::R`](R) reader structure"]
impl crate::Readable for HistWeight40to21Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_40to21::W`](W) writer structure"]
impl crate::Writable for HistWeight40to21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_40TO21 to value 0x1010_1010"]
impl crate::Resettable for HistWeight40to21Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
