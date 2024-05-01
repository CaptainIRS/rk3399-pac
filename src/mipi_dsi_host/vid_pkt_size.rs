#[doc = "Register `VID_PKT_SIZE` reader"]
pub type R = crate::R<VidPktSizeSpec>;
#[doc = "Register `VID_PKT_SIZE` writer"]
pub type W = crate::W<VidPktSizeSpec>;
#[doc = "Field `VID_PKT_SIZE` reader - vid_pkt_size\n\nThis field configures the number of pixels in a single video packet.\n\nFor 18-bit not loosely packed data types, this number must be a\n\nmultiple of 4. For YCbCr data types, it must be a multiple of 2, as\n\ndescribed in the DSI specification."]
pub type VidPktSizeR = crate::FieldReader<u16>;
#[doc = "Field `VID_PKT_SIZE` writer - vid_pkt_size\n\nThis field configures the number of pixels in a single video packet.\n\nFor 18-bit not loosely packed data types, this number must be a\n\nmultiple of 4. For YCbCr data types, it must be a multiple of 2, as\n\ndescribed in the DSI specification."]
pub type VidPktSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vid_pkt_size\n\nThis field configures the number of pixels in a single video packet.\n\nFor 18-bit not loosely packed data types, this number must be a\n\nmultiple of 4. For YCbCr data types, it must be a multiple of 2, as\n\ndescribed in the DSI specification."]
    #[inline(always)]
    pub fn vid_pkt_size(&self) -> VidPktSizeR {
        VidPktSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vid_pkt_size\n\nThis field configures the number of pixels in a single video packet.\n\nFor 18-bit not loosely packed data types, this number must be a\n\nmultiple of 4. For YCbCr data types, it must be a multiple of 2, as\n\ndescribed in the DSI specification."]
    #[inline(always)]
    #[must_use]
    pub fn vid_pkt_size(&mut self) -> VidPktSizeW<VidPktSizeSpec> {
        VidPktSizeW::new(self, 0)
    }
}
#[doc = "Video Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_pkt_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidPktSizeSpec;
impl crate::RegisterSpec for VidPktSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_pkt_size::R`](R) reader structure"]
impl crate::Readable for VidPktSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_pkt_size::W`](W) writer structure"]
impl crate::Writable for VidPktSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_PKT_SIZE to value 0"]
impl crate::Resettable for VidPktSizeSpec {
    const RESET_VALUE: u32 = 0;
}
