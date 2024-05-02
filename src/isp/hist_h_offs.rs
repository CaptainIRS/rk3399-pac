#[doc = "Register `HIST_H_OFFS` reader"]
pub type R = crate::R<HistHOffsSpec>;
#[doc = "Register `HIST_H_OFFS` writer"]
pub type W = crate::W<HistHOffsSpec>;
#[doc = "Field `hist_h_offset` reader - Horizontal offset of first window in pixels."]
pub type HistHOffsetR = crate::FieldReader<u16>;
#[doc = "Field `hist_h_offset` writer - Horizontal offset of first window in pixels."]
pub type HistHOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal offset of first window in pixels."]
    #[inline(always)]
    pub fn hist_h_offset(&self) -> HistHOffsetR {
        HistHOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal offset of first window in pixels."]
    #[inline(always)]
    #[must_use]
    pub fn hist_h_offset(&mut self) -> HistHOffsetW<HistHOffsSpec> {
        HistHOffsetW::new(self, 0)
    }
}
#[doc = "Histogram window horizontal offset for first window of \n\n\n\n25 sub- windows\n\nNote: histogram measurement is done in 25 sub-windows like the exposure \n\n\n\nmeasurement, if histogram version 3 is implemented. All earlier versions use just one \n\n\n\nwindow. \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistHOffsSpec;
impl crate::RegisterSpec for HistHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_h_offs::R`](R) reader structure"]
impl crate::Readable for HistHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_h_offs::W`](W) writer structure"]
impl crate::Writable for HistHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_H_OFFS to value 0"]
impl crate::Resettable for HistHOffsSpec {
    const RESET_VALUE: u32 = 0;
}
