#[doc = "Register `HIST_V_SIZE` reader"]
pub type R = crate::R<HistVSizeSpec>;
#[doc = "Register `HIST_V_SIZE` writer"]
pub type W = crate::W<HistVSizeSpec>;
#[doc = "Field `hist_v_size` reader - Vertical size in lines of one sub-window, if histogram\n\nversion 3 is implemented."]
pub type HistVSizeR = crate::FieldReader<u16>;
#[doc = "Field `hist_v_size` writer - Vertical size in lines of one sub-window, if histogram\n\nversion 3 is implemented."]
pub type HistVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Vertical size in lines of one sub-window, if histogram\n\nversion 3 is implemented."]
    #[inline(always)]
    pub fn hist_v_size(&self) -> HistVSizeR {
        HistVSizeR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical size in lines of one sub-window, if histogram\n\nversion 3 is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn hist_v_size(&mut self) -> HistVSizeW<HistVSizeSpec> {
        HistVSizeW::new(self, 0)
    }
}
#[doc = "Vertical (sub-)window size\n\nNote: hist_v_offset + hist_v_size x 5 should be less than or equal to the vertical size \n\n\n\nof the picture, if histogram version 3 is implemented. Otherwise hist_v_size is the vertical \n\n\n\nsize of the measurement window in lines. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistVSizeSpec;
impl crate::RegisterSpec for HistVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_v_size::R`](R) reader structure"]
impl crate::Readable for HistVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_v_size::W`](W) writer structure"]
impl crate::Writable for HistVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_V_SIZE to value 0"]
impl crate::Resettable for HistVSizeSpec {
    const RESET_VALUE: u32 = 0;
}
