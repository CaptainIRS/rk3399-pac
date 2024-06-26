#[doc = "Register `MAC_ADDR0_HI` reader"]
pub type R = crate::R<MacAddr0HiSpec>;
#[doc = "Register `MAC_ADDR0_HI` writer"]
pub type W = crate::W<MacAddr0HiSpec>;
#[doc = "Field `A47_A32` reader - MAC Address0 \\[47:32\\]\n\nThis field contains the upper 16 bits (47:32) of the 6-byte first\n\nMAC address. This is used by the MAC for filtering for received\n\nframes and for inserting the MAC address in the Transmit Flow\n\nControl (PAUSE) Frames."]
pub type A47A32R = crate::FieldReader<u16>;
#[doc = "Field `A47_A32` writer - MAC Address0 \\[47:32\\]\n\nThis field contains the upper 16 bits (47:32) of the 6-byte first\n\nMAC address. This is used by the MAC for filtering for received\n\nframes and for inserting the MAC address in the Transmit Flow\n\nControl (PAUSE) Frames."]
pub type A47A32W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]\n\nThis field contains the upper 16 bits (47:32) of the 6-byte first\n\nMAC address. This is used by the MAC for filtering for received\n\nframes and for inserting the MAC address in the Transmit Flow\n\nControl (PAUSE) Frames."]
    #[inline(always)]
    pub fn a47_a32(&self) -> A47A32R {
        A47A32R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]\n\nThis field contains the upper 16 bits (47:32) of the 6-byte first\n\nMAC address. This is used by the MAC for filtering for received\n\nframes and for inserting the MAC address in the Transmit Flow\n\nControl (PAUSE) Frames."]
    #[inline(always)]
    #[must_use]
    pub fn a47_a32(&mut self) -> A47A32W<MacAddr0HiSpec> {
        A47A32W::new(self, 0)
    }
}
#[doc = "MAC Address0 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddr0HiSpec;
impl crate::RegisterSpec for MacAddr0HiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr0_hi::R`](R) reader structure"]
impl crate::Readable for MacAddr0HiSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr0_hi::W`](W) writer structure"]
impl crate::Writable for MacAddr0HiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDR0_HI to value 0xffff"]
impl crate::Resettable for MacAddr0HiSpec {
    const RESET_VALUE: u32 = 0xffff;
}
