#[doc = "Register `DP_SINK_COUNT` reader"]
pub type R = crate::R<DpSinkCountSpec>;
#[doc = "Register `DP_SINK_COUNT` writer"]
pub type W = crate::W<DpSinkCountSpec>;
#[doc = "Field `DP_SINK_COUNT` reader - Sink Count"]
pub type DpSinkCountR = crate::FieldReader;
#[doc = "Field `DP_SINK_COUNT` writer - Sink Count"]
pub type DpSinkCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sink Count"]
    #[inline(always)]
    pub fn dp_sink_count(&self) -> DpSinkCountR {
        DpSinkCountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sink Count"]
    #[inline(always)]
    #[must_use]
    pub fn dp_sink_count(&mut self) -> DpSinkCountW<DpSinkCountSpec> {
        DpSinkCountW::new(self, 0)
    }
}
#[doc = "DP Sink Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_sink_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_sink_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpSinkCountSpec;
impl crate::RegisterSpec for DpSinkCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_sink_count::R`](R) reader structure"]
impl crate::Readable for DpSinkCountSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_sink_count::W`](W) writer structure"]
impl crate::Writable for DpSinkCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DP_SINK_COUNT to value 0"]
impl crate::Resettable for DpSinkCountSpec {
    const RESET_VALUE: u32 = 0;
}
