#[doc = "Register `HIST_WEIGHT_31TO12` reader"]
pub type R = crate::R<HistWeight31to12Spec>;
#[doc = "Register `HIST_WEIGHT_31TO12` writer"]
pub type W = crate::W<HistWeight31to12Spec>;
#[doc = "Field `hist_weight_31` reader - weighting factor for sub-window 31"]
pub type HistWeight31R = crate::FieldReader;
#[doc = "Field `hist_weight_31` writer - weighting factor for sub-window 31"]
pub type HistWeight31W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_41` reader - weighting factor for sub-window 41"]
pub type HistWeight41R = crate::FieldReader;
#[doc = "Field `hist_weight_41` writer - weighting factor for sub-window 41"]
pub type HistWeight41W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_02` reader - weighting factor for sub-window 02"]
pub type HistWeight02R = crate::FieldReader;
#[doc = "Field `hist_weight_02` writer - weighting factor for sub-window 02"]
pub type HistWeight02W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hist_weight_12` reader - weighting factor for sub-window 12"]
pub type HistWeight12R = crate::FieldReader;
#[doc = "Field `hist_weight_12` writer - weighting factor for sub-window 12"]
pub type HistWeight12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - weighting factor for sub-window 31"]
    #[inline(always)]
    pub fn hist_weight_31(&self) -> HistWeight31R {
        HistWeight31R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 41"]
    #[inline(always)]
    pub fn hist_weight_41(&self) -> HistWeight41R {
        HistWeight41R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 02"]
    #[inline(always)]
    pub fn hist_weight_02(&self) -> HistWeight02R {
        HistWeight02R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 12"]
    #[inline(always)]
    pub fn hist_weight_12(&self) -> HistWeight12R {
        HistWeight12R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - weighting factor for sub-window 31"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_31(&mut self) -> HistWeight31W<HistWeight31to12Spec> {
        HistWeight31W::new(self, 0)
    }
    #[doc = "Bits 8:12 - weighting factor for sub-window 41"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_41(&mut self) -> HistWeight41W<HistWeight31to12Spec> {
        HistWeight41W::new(self, 8)
    }
    #[doc = "Bits 16:20 - weighting factor for sub-window 02"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_02(&mut self) -> HistWeight02W<HistWeight31to12Spec> {
        HistWeight02W::new(self, 16)
    }
    #[doc = "Bits 24:28 - weighting factor for sub-window 12"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_12(&mut self) -> HistWeight12W<HistWeight31to12Spec> {
        HistWeight12W::new(self, 24)
    }
}
#[doc = "Weighting factor for sub-windows\n\nNote: Allowed value range for weight factor is 0 to 16. The resulting weight is \n\nregister_value / 16. The host software has to limit the register value for each factor to 16. \n\nWeight registers are available, if histogram version 3 is implemented. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight_31to12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight_31to12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistWeight31to12Spec;
impl crate::RegisterSpec for HistWeight31to12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight_31to12::R`](R) reader structure"]
impl crate::Readable for HistWeight31to12Spec {}
#[doc = "`write(|w| ..)` method takes [`hist_weight_31to12::W`](W) writer structure"]
impl crate::Writable for HistWeight31to12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT_31TO12 to value 0x1010_1010"]
impl crate::Resettable for HistWeight31to12Spec {
    const RESET_VALUE: u32 = 0x1010_1010;
}
