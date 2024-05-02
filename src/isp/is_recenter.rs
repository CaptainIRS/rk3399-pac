#[doc = "Register `IS_RECENTER` reader"]
pub type R = crate::R<IsRecenterSpec>;
#[doc = "Register `IS_RECENTER` writer"]
pub type W = crate::W<IsRecenterSpec>;
#[doc = "Field `is_recenter` reader - 000: recenter feature switched off\n\n1..7: recentering by (cur_h/v_offs-H/V_OFFS)/2^RECENTER\n\n"]
pub type IsRecenterR = crate::FieldReader;
#[doc = "Field `is_recenter` writer - 000: recenter feature switched off\n\n1..7: recentering by (cur_h/v_offs-H/V_OFFS)/2^RECENTER\n\n"]
pub type IsRecenterW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 000: recenter feature switched off\n\n1..7: recentering by (cur_h/v_offs-H/V_OFFS)/2^RECENTER\n\n"]
    #[inline(always)]
    pub fn is_recenter(&self) -> IsRecenterR {
        IsRecenterR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 000: recenter feature switched off\n\n1..7: recentering by (cur_h/v_offs-H/V_OFFS)/2^RECENTER\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_recenter(&mut self) -> IsRecenterW<IsRecenterSpec> {
        IsRecenterW::new(self, 0)
    }
}
#[doc = "Recenter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_recenter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_recenter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsRecenterSpec;
impl crate::RegisterSpec for IsRecenterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_recenter::R`](R) reader structure"]
impl crate::Readable for IsRecenterSpec {}
#[doc = "`write(|w| ..)` method takes [`is_recenter::W`](W) writer structure"]
impl crate::Writable for IsRecenterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_RECENTER to value 0"]
impl crate::Resettable for IsRecenterSpec {
    const RESET_VALUE: u32 = 0;
}
