#[doc = "Register `HIST_V_OFFS` reader"]
pub type R = crate::R<HistVOffsSpec>;
#[doc = "Register `HIST_V_OFFS` writer"]
pub type W = crate::W<HistVOffsSpec>;
#[doc = "Field `hist_v_offset` reader - Vertical offset of first window in pixels."]
pub type HistVOffsetR = crate::FieldReader<u16>;
#[doc = "Field `hist_v_offset` writer - Vertical offset of first window in pixels."]
pub type HistVOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical offset of first window in pixels."]
    #[inline(always)]
    pub fn hist_v_offset(&self) -> HistVOffsetR {
        HistVOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical offset of first window in pixels."]
    #[inline(always)]
    #[must_use]
    pub fn hist_v_offset(&mut self) -> HistVOffsetW<HistVOffsSpec> {
        HistVOffsetW::new(self, 0)
    }
}
#[doc = "Histogram window vertical offset for first window of 25 sub-windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one window. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistVOffsSpec;
impl crate::RegisterSpec for HistVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_v_offs::R`](R) reader structure"]
impl crate::Readable for HistVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_v_offs::W`](W) writer structure"]
impl crate::Writable for HistVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_V_OFFS to value 0"]
impl crate::Resettable for HistVOffsSpec {
    const RESET_VALUE: u32 = 0;
}
