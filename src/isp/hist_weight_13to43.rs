#[doc = "Register `HIST_WEIGHT_13TO43` reader"]
pub type R = crate::R<HistWeight13to43Spec>;
#[doc = "Register `HIST_WEIGHT_13TO43` writer"]
pub type W = crate::W<HistWeight13to43Spec>;
#[doc = "Field `hist_weight_13` reader - weighting factor for sub-window 13"]
pub type HistWeight13R = crate::FieldReader;
#[doc = "Field `hist_weight_13` writer - weighting factor for sub-window 13"]
pub type HistWeight13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_23` reader - weighting factor for sub-window 23"]
pub type HistWeight23R = crate::FieldReader;
#[doc = "Field `hist_weight_23` writer - weighting factor for sub-window 23"]
pub type HistWeight23W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_33` reader - weighting factor for sub-window 33"]
pub type HistWeight33R = crate::FieldReader;
#[doc = "Field `hist_weight_33` writer - weighting factor for sub-window 33"]
pub type HistWeight33W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_43` reader - weighting factor for sub-window 43"]
pub type HistWeight43R = crate::FieldReader;
#[doc = "Field `hist_weight_43` writer - weighting factor for sub-window 43"]
pub type HistWeight43W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 13"]
    #[inline(always)]
    pub fn hist_weight_13(&self) -> HistWeight13R {
        HistWeight13R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 23"]
    #[inline(always)]
    pub fn hist_weight_23(&self) -> HistWeight23R {
        HistWeight23R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 33"]
    #[inline(always)]
    pub fn hist_weight_33(&self) -> HistWeight33R {
        HistWeight33R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 43"]
    #[inline(always)]
    pub fn hist_weight_43(&self) -> HistWeight43R {
        HistWeight43R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 13"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_13(&mut self) -> HistWeight13W<HistWeight13to43Spec> {
        HistWeight13W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 23"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_23(&mut self) -> HistWeight23W<HistWeight13to43Spec> {
        HistWeight23W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 33"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_33(&mut self) -> HistWeight33W<HistWeight13to43Spec> {
        HistWeight33W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 43"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_43(&mut self) -> HistWeight43W<HistWeight13to43Spec> {
        HistWeight43W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_13to43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_13to43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight13to43Spec;
impl crate::RegisterSpec for HistWeight13to43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_13to43::R`](R) reader structure"]
impl crate::Readable for HistWeight13to43Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_13to43::W`](W) writer structure"]
impl crate::Writable for HistWeight13to43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_13TO43 to value 0x1010_1010"]
impl crate::Resettable for HistWeight13to43Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
