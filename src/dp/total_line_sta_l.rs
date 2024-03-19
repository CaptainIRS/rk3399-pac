#[doc = "Register `TOTAL_LINE_STA_L` reader"]
pub type R = crate::R<TotalLineStaLSpec>;
#[doc = "Register `TOTAL_LINE_STA_L` writer"]
pub type W = crate::W<TotalLineStaLSpec>;
#[doc = "Field `TOTAL_LINE_STA_L` reader - TOTAL_LINE \\[7:0\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type TotalLineStaLR = crate::FieldReader;
#[doc = "Field `TOTAL_LINE_STA_L` writer - TOTAL_LINE \\[7:0\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type TotalLineStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TOTAL_LINE \\[7:0\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn total_line_sta_l(&self) -> TotalLineStaLR {
        TotalLineStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TOTAL_LINE \\[7:0\\]
which is detected by video \n\ncapture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn total_line_sta_l(&mut self) -> TotalLineStaLW<TotalLineStaLSpec> {
        TotalLineStaLW::new(self, 0)
    }
}
#[doc = "Total Line Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalLineStaLSpec;
impl crate::RegisterSpec for TotalLineStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_line_sta_l::R`](R) reader structure"]
impl crate::Readable for TotalLineStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`total_line_sta_l::W`](W) writer structure"]
impl crate::Writable for TotalLineStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOTAL_LINE_STA_L to value 0x01"]
impl crate::Resettable for TotalLineStaLSpec {
    const RESET_VALUE: u32 = 0x01;
}
