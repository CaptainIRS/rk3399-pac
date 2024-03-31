#[doc = "Register `GFIFOPRIDBC` reader"]
pub type R = crate::R<GfifopridbcSpec>;
#[doc = "Register `GFIFOPRIDBC` writer"]
pub type W = crate::W<GfifopridbcSpec>;
#[doc = "Field `GFIFOPRIDBC` reader - Host DbC DMA priority\n\nThis register specifies the relative priority of the RXFIFOs and\n\nTXFIFOs associated with the DbC mode. It overrides the priority\n\nassigned in the corresponding indexes of the Host RXFIFO and\n\nTXFIFO DMA priority registers, when the DbC mode is enabled.\n\nPriority settings are specified in relation to the low-priority SS\n\nspeed group:\n\n1. Normal priority indicates that the DbC FIFOs are considered\n\nidentical to the Host SS low-priority FIFOs.\n\n2. Low priority indicates that the DbC FIFOs are considered to\n\nhave lower priority than all Host SS FIFOs.\n\n3. High priority indicates that the DbC FIFOs are considered\n\nhigher priority than the Host SS low-priority FIFOs but lower\n\npriority than the Host SS high-priority FIFOs.\n\nThis register is present only when the core is configured to\n\noperate in Host Debug Capability (DbC) mode."]
pub type GfifopridbcR = crate::FieldReader;
#[doc = "Field `GFIFOPRIDBC` writer - Host DbC DMA priority\n\nThis register specifies the relative priority of the RXFIFOs and\n\nTXFIFOs associated with the DbC mode. It overrides the priority\n\nassigned in the corresponding indexes of the Host RXFIFO and\n\nTXFIFO DMA priority registers, when the DbC mode is enabled.\n\nPriority settings are specified in relation to the low-priority SS\n\nspeed group:\n\n1. Normal priority indicates that the DbC FIFOs are considered\n\nidentical to the Host SS low-priority FIFOs.\n\n2. Low priority indicates that the DbC FIFOs are considered to\n\nhave lower priority than all Host SS FIFOs.\n\n3. High priority indicates that the DbC FIFOs are considered\n\nhigher priority than the Host SS low-priority FIFOs but lower\n\npriority than the Host SS high-priority FIFOs.\n\nThis register is present only when the core is configured to\n\noperate in Host Debug Capability (DbC) mode."]
pub type GfifopridbcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Host DbC DMA priority\n\nThis register specifies the relative priority of the RXFIFOs and\n\nTXFIFOs associated with the DbC mode. It overrides the priority\n\nassigned in the corresponding indexes of the Host RXFIFO and\n\nTXFIFO DMA priority registers, when the DbC mode is enabled.\n\nPriority settings are specified in relation to the low-priority SS\n\nspeed group:\n\n1. Normal priority indicates that the DbC FIFOs are considered\n\nidentical to the Host SS low-priority FIFOs.\n\n2. Low priority indicates that the DbC FIFOs are considered to\n\nhave lower priority than all Host SS FIFOs.\n\n3. High priority indicates that the DbC FIFOs are considered\n\nhigher priority than the Host SS low-priority FIFOs but lower\n\npriority than the Host SS high-priority FIFOs.\n\nThis register is present only when the core is configured to\n\noperate in Host Debug Capability (DbC) mode."]
    #[inline(always)]
    pub fn gfifopridbc(&self) -> GfifopridbcR {
        GfifopridbcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Host DbC DMA priority\n\nThis register specifies the relative priority of the RXFIFOs and\n\nTXFIFOs associated with the DbC mode. It overrides the priority\n\nassigned in the corresponding indexes of the Host RXFIFO and\n\nTXFIFO DMA priority registers, when the DbC mode is enabled.\n\nPriority settings are specified in relation to the low-priority SS\n\nspeed group:\n\n1. Normal priority indicates that the DbC FIFOs are considered\n\nidentical to the Host SS low-priority FIFOs.\n\n2. Low priority indicates that the DbC FIFOs are considered to\n\nhave lower priority than all Host SS FIFOs.\n\n3. High priority indicates that the DbC FIFOs are considered\n\nhigher priority than the Host SS low-priority FIFOs but lower\n\npriority than the Host SS high-priority FIFOs.\n\nThis register is present only when the core is configured to\n\noperate in Host Debug Capability (DbC) mode."]
    #[inline(always)]
    #[must_use]
    pub fn gfifopridbc(&mut self) -> GfifopridbcW<GfifopridbcSpec> {
        GfifopridbcW::new(self, 0)
    }
}
#[doc = "Global Host Debug Capability DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfifopridbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfifopridbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfifopridbcSpec;
impl crate::RegisterSpec for GfifopridbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfifopridbc::R`](R) reader structure"]
impl crate::Readable for GfifopridbcSpec {}
#[doc = "`write(|w| ..)` method takes [`gfifopridbc::W`](W) writer structure"]
impl crate::Writable for GfifopridbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFIFOPRIDBC to value 0"]
impl crate::Resettable for GfifopridbcSpec {
    const RESET_VALUE: u32 = 0;
}
