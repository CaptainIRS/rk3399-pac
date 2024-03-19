#[doc = "Register `H_SYNC_STA_H` reader"]
pub type R = crate::R<HSyncStaHSpec>;
#[doc = "Register `H_SYNC_STA_H` writer"]
pub type W = crate::W<HSyncStaHSpec>;
#[doc = "Field `H_SYNC_STA_H` reader - H_SYNC \\[11:8\\]
(horizon sync width) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is high. \n\nAnd STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type HSyncStaHR = crate::FieldReader;
#[doc = "Field `H_SYNC_STA_H` writer - H_SYNC \\[11:8\\]
(horizon sync width) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is high. \n\nAnd STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type HSyncStaHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - H_SYNC \\[11:8\\]
(horizon sync width) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is high. \n\nAnd STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn h_sync_sta_h(&self) -> HSyncStaHR {
        HSyncStaHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - H_SYNC \\[11:8\\]
(horizon sync width) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is high. \n\nAnd STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_sync_sta_h(&mut self) -> HSyncStaHW<HSyncStaHSpec> {
        HSyncStaHW::new(self, 0)
    }
}
#[doc = "Horizon Sync Width Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSyncStaHSpec;
impl crate::RegisterSpec for HSyncStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_sync_sta_h::R`](R) reader structure"]
impl crate::Readable for HSyncStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`h_sync_sta_h::W`](W) writer structure"]
impl crate::Writable for HSyncStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets H_SYNC_STA_H to value 0"]
impl crate::Resettable for HSyncStaHSpec {
    const RESET_VALUE: u32 = 0;
}
