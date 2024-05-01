#[doc = "Register `VID_HBP_TIME` reader"]
pub type R = crate::R<VidHbpTimeSpec>;
#[doc = "Register `VID_HBP_TIME` writer"]
pub type W = crate::W<VidHbpTimeSpec>;
#[doc = "Field `VID_HBP_TIME` reader - vid_hbp_time\n\nThis field configures the Horizontal Back Porch period in lane byte\n\nclock cycles."]
pub type VidHbpTimeR = crate::FieldReader<u16>;
#[doc = "Field `VID_HBP_TIME` writer - vid_hbp_time\n\nThis field configures the Horizontal Back Porch period in lane byte\n\nclock cycles."]
pub type VidHbpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - vid_hbp_time\n\nThis field configures the Horizontal Back Porch period in lane byte\n\nclock cycles."]
    #[inline(always)]
    pub fn vid_hbp_time(&self) -> VidHbpTimeR {
        VidHbpTimeR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - vid_hbp_time\n\nThis field configures the Horizontal Back Porch period in lane byte\n\nclock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn vid_hbp_time(&mut self) -> VidHbpTimeW<VidHbpTimeSpec> {
        VidHbpTimeW::new(self, 0)
    }
}
#[doc = "Register0005 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hbp_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hbp_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidHbpTimeSpec;
impl crate::RegisterSpec for VidHbpTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hbp_time::R`](R) reader structure"]
impl crate::Readable for VidHbpTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_hbp_time::W`](W) writer structure"]
impl crate::Writable for VidHbpTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_HBP_TIME to value 0"]
impl crate::Resettable for VidHbpTimeSpec {
    const RESET_VALUE: u32 = 0;
}
