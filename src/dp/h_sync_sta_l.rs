#[doc = "Register `H_SYNC_STA_L` reader"]
pub type R = crate::R<HSyncStaLSpec>;
#[doc = "Register `H_SYNC_STA_L` writer"]
pub type W = crate::W<HSyncStaLSpec>;
#[doc = "Field `H_SYNC_STA_L` reader - H_SYNC \\[7:0\\]
(horizon sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HSyncStaLR = crate::FieldReader;
#[doc = "Field `H_SYNC_STA_L` writer - H_SYNC \\[7:0\\]
(horizon sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HSyncStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_SYNC \\[7:0\\]
(horizon sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn h_sync_sta_l(&self) -> HSyncStaLR {
        HSyncStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_SYNC \\[7:0\\]
(horizon sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_sync_sta_l(&mut self) -> HSyncStaLW<HSyncStaLSpec> {
        HSyncStaLW::new(self, 0)
    }
}
#[doc = "Horizon Sync Width Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSyncStaLSpec;
impl crate::RegisterSpec for HSyncStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_sync_sta_l::R`](R) reader structure"]
impl crate::Readable for HSyncStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_sync_sta_l::W`](W) writer structure"]
impl crate::Writable for HSyncStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_SYNC_STA_L to value 0"]
impl crate::Resettable for HSyncStaLSpec {
    const RESET_VALUE: u32 = 0;
}
