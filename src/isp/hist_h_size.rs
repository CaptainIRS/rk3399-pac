#[doc = "Register `HIST_H_SIZE` reader"]
pub type R = crate::R<HistHSizeSpec>;
#[doc = "Register `HIST_H_SIZE` writer"]
pub type W = crate::W<HistHSizeSpec>;
#[doc = "Field `hist_h_size` reader - Horizontal size in pixels of one sub-window, if\n\nhistogram version 3 is implemented."]
pub type HistHSizeR = crate::FieldReader<u16>;
#[doc = "Field `hist_h_size` writer - Horizontal size in pixels of one sub-window, if\n\nhistogram version 3 is implemented."]
pub type HistHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Horizontal size in pixels of one sub-window, if\n\nhistogram version 3 is implemented."]
    #[inline(always)]
    pub fn hist_h_size(&self) -> HistHSizeR {
        HistHSizeR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Horizontal size in pixels of one sub-window, if\n\nhistogram version 3 is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn hist_h_size(&mut self) -> HistHSizeW<HistHSizeSpec> {
        HistHSizeW::new(self, 0)
    }
}
#[doc = "Horizontal (sub-)window size\n\nNote: hist_h_offset + hist_h_size x 5 should be less than or equal to the horizontal size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_h_size is the horizontal \n\n\n\nsize of the measurement window in pixels. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistHSizeSpec;
impl crate::RegisterSpec for HistHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_h_size::R`](R) reader structure"]
impl crate::Readable for HistHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_h_size::W`](W) writer structure"]
impl crate::Writable for HistHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_H_SIZE to value 0"]
impl crate::Resettable for HistHSizeSpec {
    const RESET_VALUE: u32 = 0;
}
