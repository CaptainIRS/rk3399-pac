#[doc = "Register `USB3_GFIFOPRIDBC` reader"]
pub type R = crate::R<Usb3GfifopridbcSpec>;
#[doc = "Register `USB3_GFIFOPRIDBC` writer"]
pub type W = crate::W<Usb3GfifopridbcSpec>;
#[doc = "Field `GFIFOPRIDBC` reader - Host DbC DMA priority This register specifies the relative priority of the RXFIFOs and TXFIFOs associated with the DbC mode. It overrides the priority assigned in the corresponding indexes of the Host RXFIFO and TXFIFO DMA priority registers, when the DbC mode is enabled. Priority settings are specified in relation to the low-priority SS speed group: 1. Normal priority indicates that the DbC FIFOs are considered identical to the Host SS low-priority FIFOs. 2. Low priority indicates that the DbC FIFOs are considered to have lower priority than all Host SS FIFOs. 3. High priority indicates that the DbC FIFOs are considered higher priority than the Host SS low-priority FIFOs but lower priority than the Host SS high-priority FIFOs. This register is present only when the core is configured to operate in Host Debug Capability (DbC) mode."]
pub type GfifopridbcR = crate::FieldReader;
#[doc = "Field `GFIFOPRIDBC` writer - Host DbC DMA priority This register specifies the relative priority of the RXFIFOs and TXFIFOs associated with the DbC mode. It overrides the priority assigned in the corresponding indexes of the Host RXFIFO and TXFIFO DMA priority registers, when the DbC mode is enabled. Priority settings are specified in relation to the low-priority SS speed group: 1. Normal priority indicates that the DbC FIFOs are considered identical to the Host SS low-priority FIFOs. 2. Low priority indicates that the DbC FIFOs are considered to have lower priority than all Host SS FIFOs. 3. High priority indicates that the DbC FIFOs are considered higher priority than the Host SS low-priority FIFOs but lower priority than the Host SS high-priority FIFOs. This register is present only when the core is configured to operate in Host Debug Capability (DbC) mode."]
pub type GfifopridbcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Host DbC DMA priority This register specifies the relative priority of the RXFIFOs and TXFIFOs associated with the DbC mode. It overrides the priority assigned in the corresponding indexes of the Host RXFIFO and TXFIFO DMA priority registers, when the DbC mode is enabled. Priority settings are specified in relation to the low-priority SS speed group: 1. Normal priority indicates that the DbC FIFOs are considered identical to the Host SS low-priority FIFOs. 2. Low priority indicates that the DbC FIFOs are considered to have lower priority than all Host SS FIFOs. 3. High priority indicates that the DbC FIFOs are considered higher priority than the Host SS low-priority FIFOs but lower priority than the Host SS high-priority FIFOs. This register is present only when the core is configured to operate in Host Debug Capability (DbC) mode."]
    #[inline(always)]
    pub fn gfifopridbc(&self) -> GfifopridbcR {
        GfifopridbcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Host DbC DMA priority This register specifies the relative priority of the RXFIFOs and TXFIFOs associated with the DbC mode. It overrides the priority assigned in the corresponding indexes of the Host RXFIFO and TXFIFO DMA priority registers, when the DbC mode is enabled. Priority settings are specified in relation to the low-priority SS speed group: 1. Normal priority indicates that the DbC FIFOs are considered identical to the Host SS low-priority FIFOs. 2. Low priority indicates that the DbC FIFOs are considered to have lower priority than all Host SS FIFOs. 3. High priority indicates that the DbC FIFOs are considered higher priority than the Host SS low-priority FIFOs but lower priority than the Host SS high-priority FIFOs. This register is present only when the core is configured to operate in Host Debug Capability (DbC) mode."]
    #[inline(always)]
    #[must_use]
    pub fn gfifopridbc(&mut self) -> GfifopridbcW<Usb3GfifopridbcSpec> {
        GfifopridbcW::new(self, 0)
    }
}
#[doc = "Global Host Debug Capability DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gfifopridbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gfifopridbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GfifopridbcSpec;
impl crate::RegisterSpec for Usb3GfifopridbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gfifopridbc::R`](R) reader structure"]
impl crate::Readable for Usb3GfifopridbcSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gfifopridbc::W`](W) writer structure"]
impl crate::Writable for Usb3GfifopridbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GFIFOPRIDBC to value 0"]
impl crate::Resettable for Usb3GfifopridbcSpec {
    const RESET_VALUE: u32 = 0;
}
