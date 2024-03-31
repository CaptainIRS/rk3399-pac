#[doc = "Register `MAC_ADDR0_LO` reader"]
pub type R = crate::R<MacAddr0LoSpec>;
#[doc = "Register `MAC_ADDR0_LO` writer"]
pub type W = crate::W<MacAddr0LoSpec>;
#[doc = "Field `A31_A0` reader - MAC Address0 \\[31:0\\]\n\nThis field contains the lower 32 bits of the 6-byte first MAC\n\naddress. This is used by the MAC for filtering for received frames\n\nand for inserting the MAC address in the Transmit Flow Control\n\n(PAUSE) Frames."]
pub type A31A0R = crate::FieldReader<u32>;
#[doc = "Field `A31_A0` writer - MAC Address0 \\[31:0\\]\n\nThis field contains the lower 32 bits of the 6-byte first MAC\n\naddress. This is used by the MAC for filtering for received frames\n\nand for inserting the MAC address in the Transmit Flow Control\n\n(PAUSE) Frames."]
pub type A31A0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]\n\nThis field contains the lower 32 bits of the 6-byte first MAC\n\naddress. This is used by the MAC for filtering for received frames\n\nand for inserting the MAC address in the Transmit Flow Control\n\n(PAUSE) Frames."]
    #[inline(always)]
    pub fn a31_a0(&self) -> A31A0R {
        A31A0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]\n\nThis field contains the lower 32 bits of the 6-byte first MAC\n\naddress. This is used by the MAC for filtering for received frames\n\nand for inserting the MAC address in the Transmit Flow Control\n\n(PAUSE) Frames."]
    #[inline(always)]
    #[must_use]
    pub fn a31_a0(&mut self) -> A31A0W<MacAddr0LoSpec> {
        A31A0W::new(self, 0)
    }
}
#[doc = "MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddr0LoSpec;
impl crate::RegisterSpec for MacAddr0LoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr0_lo::R`](R) reader structure"]
impl crate::Readable for MacAddr0LoSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr0_lo::W`](W) writer structure"]
impl crate::Writable for MacAddr0LoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDR0_LO to value 0xffff_ffff"]
impl crate::Resettable for MacAddr0LoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
