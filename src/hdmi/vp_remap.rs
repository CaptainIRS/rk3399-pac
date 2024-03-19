#[doc = "Register `VP_REMAP` reader"]
pub type R = crate::R<VpRemapSpec>;
#[doc = "Register `VP_REMAP` writer"]
pub type W = crate::W<VpRemapSpec>;
#[doc = "Field `YCC422_SIZE` reader - YCC 422 remap input video size ycc422_size\\[1:0\\]\n\n00b: YCC 422 16-bit input video (8 bits per\n\ncomponent) 01b: YCC 422 20-bit input video (10\n\nbits per component) 10b: YCC 422 24-bit input\n\nvideo (12 bits per component) 11b: Reserved. Not\n\nused"]
pub type Ycc422SizeR = crate::FieldReader;
#[doc = "Field `YCC422_SIZE` writer - YCC 422 remap input video size ycc422_size\\[1:0\\]\n\n00b: YCC 422 16-bit input video (8 bits per\n\ncomponent) 01b: YCC 422 20-bit input video (10\n\nbits per component) 10b: YCC 422 24-bit input\n\nvideo (12 bits per component) 11b: Reserved. Not\n\nused"]
pub type Ycc422SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - YCC 422 remap input video size ycc422_size\\[1:0\\]\n\n00b: YCC 422 16-bit input video (8 bits per\n\ncomponent) 01b: YCC 422 20-bit input video (10\n\nbits per component) 10b: YCC 422 24-bit input\n\nvideo (12 bits per component) 11b: Reserved. Not\n\nused"]
    #[inline(always)]
    pub fn ycc422_size(&self) -> Ycc422SizeR {
        Ycc422SizeR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - YCC 422 remap input video size ycc422_size\\[1:0\\]\n\n00b: YCC 422 16-bit input video (8 bits per\n\ncomponent) 01b: YCC 422 20-bit input video (10\n\nbits per component) 10b: YCC 422 24-bit input\n\nvideo (12 bits per component) 11b: Reserved. Not\n\nused"]
    #[inline(always)]
    #[must_use]
    pub fn ycc422_size(&mut self) -> Ycc422SizeW<VpRemapSpec> {
        Ycc422SizeW::new(self, 0)
    }
}
#[doc = "Video Packetizer YCC422 Remapping Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vp_remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpRemapSpec;
impl crate::RegisterSpec for VpRemapSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_remap::R`](R) reader structure"]
impl crate::Readable for VpRemapSpec {}
#[doc = "`write(|w| ..)` method takes [`vp_remap::W`](W) writer structure"]
impl crate::Writable for VpRemapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets VP_REMAP to value 0"]
impl crate::Resettable for VpRemapSpec {
    const RESET_VALUE: u8 = 0;
}
