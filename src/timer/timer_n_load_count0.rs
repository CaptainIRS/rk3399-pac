#[doc = "Register `TIMER_n_LOAD_COUNT0` reader"]
pub type R = crate::R<TimerNLoadCount0Spec>;
#[doc = "Register `TIMER_n_LOAD_COUNT0` writer"]
pub type W = crate::W<TimerNLoadCount0Spec>;
#[doc = "Field `LOAD_COUNT_LOW_BITS` reader - Low 32 bits value to be loaded into Timer n."]
pub type LoadCountLowBitsR = crate::FieldReader<u32>;
#[doc = "Field `LOAD_COUNT_LOW_BITS` writer - Low 32 bits value to be loaded into Timer n."]
pub type LoadCountLowBitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    pub fn load_count_low_bits(&self) -> LoadCountLowBitsR {
        LoadCountLowBitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    #[must_use]
    pub fn load_count_low_bits(&mut self) -> LoadCountLowBitsW<TimerNLoadCount0Spec> {
        LoadCountLowBitsW::new(self, 0)
    }
}
#[doc = "Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_load_count0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_load_count0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerNLoadCount0Spec;
impl crate::RegisterSpec for TimerNLoadCount0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_n_load_count0::R`](R) reader structure"]
impl crate::Readable for TimerNLoadCount0Spec {}
#[doc = "`write(|w| ..)` method takes [`timer_n_load_count0::W`](W) writer structure"]
impl crate::Writable for TimerNLoadCount0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_n_LOAD_COUNT0 to value 0"]
impl crate::Resettable for TimerNLoadCount0Spec {
    const RESET_VALUE: u32 = 0;
}
