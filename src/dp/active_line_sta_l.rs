#[doc = "Register `ACTIVE_LINE_STA_L` reader"]
pub type R = crate::R<ActiveLineStaLSpec>;
#[doc = "Register `ACTIVE_LINE_STA_L` writer"]
pub type W = crate::W<ActiveLineStaLSpec>;
#[doc = "Field `ACTIVE_LINE_STA_L` reader - ACTIVE_LINE \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type ActiveLineStaLR = crate::FieldReader;
#[doc = "Field `ACTIVE_LINE_STA_L` writer - ACTIVE_LINE \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type ActiveLineStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ACTIVE_LINE \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn active_line_sta_l(&self) -> ActiveLineStaLR {
        ActiveLineStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACTIVE_LINE \\[7:0\\]
which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn active_line_sta_l(&mut self) -> ActiveLineStaLW<ActiveLineStaLSpec> {
        ActiveLineStaLW::new(self, 0)
    }
}
#[doc = "Active Line Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveLineStaLSpec;
impl crate::RegisterSpec for ActiveLineStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_line_sta_l::R`](R) reader structure"]
impl crate::Readable for ActiveLineStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`active_line_sta_l::W`](W) writer structure"]
impl crate::Writable for ActiveLineStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ACTIVE_LINE_STA_L to value 0"]
impl crate::Resettable for ActiveLineStaLSpec {
    const RESET_VALUE: u32 = 0;
}
