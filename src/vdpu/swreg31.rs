#[doc = "Register `SWREG31` reader"]
pub type R = crate::R<Swreg31Spec>;
#[doc = "Register `SWREG31` writer"]
pub type W = crate::W<Swreg31Spec>;
#[doc = "Field `SW_CONT_THR0` reader - the threshold value 0 for contrast adjust"]
pub type SwContThr0R = crate::FieldReader;
#[doc = "Field `SW_CONT_THR0` writer - the threshold value 0 for contrast adjust"]
pub type SwContThr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_CONT_THR1` reader - the threshold value 1 for contrast adjust"]
pub type SwContThr1R = crate::FieldReader;
#[doc = "Field `SW_CONT_THR1` writer - the threshold value 1 for contrast adjust"]
pub type SwContThr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the threshold value 0 for contrast adjust"]
    #[inline(always)]
    pub fn sw_cont_thr0(&self) -> SwContThr0R {
        SwContThr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the threshold value 1 for contrast adjust"]
    #[inline(always)]
    pub fn sw_cont_thr1(&self) -> SwContThr1R {
        SwContThr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the threshold value 0 for contrast adjust"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cont_thr0(&mut self) -> SwContThr0W<Swreg31Spec> {
        SwContThr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - the threshold value 1 for contrast adjust"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cont_thr1(&mut self) -> SwContThr1W<Swreg31Spec> {
        SwContThr1W::new(self, 8)
    }
}
#[doc = "contrast adjust threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg31Spec;
impl crate::RegisterSpec for Swreg31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg31::R`](R) reader structure"]
impl crate::Readable for Swreg31Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg31::W`](W) writer structure"]
impl crate::Writable for Swreg31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG31 to value 0"]
impl crate::Resettable for Swreg31Spec {
    const RESET_VALUE: u32 = 0;
}
