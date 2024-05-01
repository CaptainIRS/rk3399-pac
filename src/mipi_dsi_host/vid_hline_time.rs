#[doc = "Register `VID_HLINE_TIME` reader"]
pub type R = crate::R<VidHlineTimeSpec>;
#[doc = "Register `VID_HLINE_TIME` writer"]
pub type W = crate::W<VidHlineTimeSpec>;
#[doc = "Field `VID_HLINE_TIME` reader - vid_hline_time\n\nThis field configures the size of the total line time\n\n(HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
pub type VidHlineTimeR = crate::FieldReader<u16>;
#[doc = "Field `VID_HLINE_TIME` writer - vid_hline_time\n\nThis field configures the size of the total line time\n\n(HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
pub type VidHlineTimeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - vid_hline_time\n\nThis field configures the size of the total line time\n\n(HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
    #[inline(always)]
    pub fn vid_hline_time(&self) -> VidHlineTimeR {
        VidHlineTimeR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - vid_hline_time\n\nThis field configures the size of the total line time\n\n(HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn vid_hline_time(&mut self) -> VidHlineTimeW<VidHlineTimeSpec> {
        VidHlineTimeW::new(self, 0)
    }
}
#[doc = "Line Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hline_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidHlineTimeSpec;
impl crate::RegisterSpec for VidHlineTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hline_time::R`](R) reader structure"]
impl crate::Readable for VidHlineTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_hline_time::W`](W) writer structure"]
impl crate::Writable for VidHlineTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_HLINE_TIME to value 0"]
impl crate::Resettable for VidHlineTimeSpec {
    const RESET_VALUE: u32 = 0;
}
