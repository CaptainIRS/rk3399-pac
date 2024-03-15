#[doc = "Register `V_SYNC_STA` reader"]
pub type R = crate::R<VSyncStaSpec>;
#[doc = "Register `V_SYNC_STA` writer"]
pub type W = crate::W<VSyncStaSpec>;
#[doc = "Field `V_SYNC_STA` reader - V_SYNC_WIDTH (vertical sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type VSyncStaR = crate::FieldReader;
#[doc = "Field `V_SYNC_STA` writer - V_SYNC_WIDTH (vertical sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type VSyncStaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - V_SYNC_WIDTH (vertical sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn v_sync_sta(&self) -> VSyncStaR {
        VSyncStaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - V_SYNC_WIDTH (vertical sync width) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn v_sync_sta(&mut self) -> VSyncStaW<VSyncStaSpec> {
        VSyncStaW::new(self, 0)
    }
}
#[doc = "Vertical Sync Width Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_sync_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_sync_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VSyncStaSpec;
impl crate::RegisterSpec for VSyncStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v_sync_sta::R`](R) reader structure"]
impl crate::Readable for VSyncStaSpec {}
#[doc = "`write(|w| ..)` method takes [`v_sync_sta::W`](W) writer structure"]
impl crate::Writable for VSyncStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets V_SYNC_STA to value 0"]
impl crate::Resettable for VSyncStaSpec {
    const RESET_VALUE: u32 = 0;
}
