#[doc = "Register `TOTAL_LINE_STA_H` reader"]
pub type R = crate::R<TotalLineStaHSpec>;
#[doc = "Register `TOTAL_LINE_STA_H` writer"]
pub type W = crate::W<TotalLineStaHSpec>;
#[doc = "Field `TOTAL_LINE_STA_H` reader - TOTAL_LINE \\[11:8\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type TotalLineStaHR = crate::FieldReader;
#[doc = "Field `TOTAL_LINE_STA_H` writer - TOTAL_LINE \\[11:8\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type TotalLineStaHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - TOTAL_LINE \\[11:8\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn total_line_sta_h(&self) -> TotalLineStaHR {
        TotalLineStaHR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - TOTAL_LINE \\[11:8\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn total_line_sta_h(&mut self) -> TotalLineStaHW<TotalLineStaHSpec> {
        TotalLineStaHW::new(self, 0)
    }
}
#[doc = "Total Line Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalLineStaHSpec;
impl crate::RegisterSpec for TotalLineStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_line_sta_h::R`](R) reader structure"]
impl crate::Readable for TotalLineStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`total_line_sta_h::W`](W) writer structure"]
impl crate::Writable for TotalLineStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOTAL_LINE_STA_H to value 0"]
impl crate::Resettable for TotalLineStaHSpec {
    const RESET_VALUE: u32 = 0;
}
