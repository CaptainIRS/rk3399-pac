#[doc = "Register `ACTIVE_LINE_STA_H` reader"]
pub type R = crate::R<ActiveLineStaHSpec>;
#[doc = "Register `ACTIVE_LINE_STA_H` writer"]
pub type W = crate::W<ActiveLineStaHSpec>;
#[doc = "Field `ACTIVE_LINE_STA_H` reader - ACTIVE_LINE \\[11:8\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
pub type ActiveLineStaHR = crate::FieldReader;
#[doc = "Field `ACTIVE_LINE_STA_H` writer - ACTIVE_LINE \\[11:8\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
pub type ActiveLineStaHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ACTIVE_LINE \\[11:8\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
    #[inline(always)]
    pub fn active_line_sta_h(&self) -> ActiveLineStaHR {
        ActiveLineStaHR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ACTIVE_LINE \\[11:8\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
    #[inline(always)]
    #[must_use]
    pub fn active_line_sta_h(&mut self) -> ActiveLineStaHW<ActiveLineStaHSpec> {
        ActiveLineStaHW::new(self, 0)
    }
}
#[doc = "Active Line Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveLineStaHSpec;
impl crate::RegisterSpec for ActiveLineStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_line_sta_h::R`](R) reader structure"]
impl crate::Readable for ActiveLineStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`active_line_sta_h::W`](W) writer structure"]
impl crate::Writable for ActiveLineStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
#[doc = "`reset()` method sets ACTIVE_LINE_STA_H to value 0"]
impl crate::Resettable for ActiveLineStaHSpec {
    const RESET_VALUE: u32 = 0;
}
