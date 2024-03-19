#[doc = "Register `USB3_GDBGFIFOSPACE` reader"]
pub type R = crate::R<Usb3GdbgfifospaceSpec>;
#[doc = "Register `USB3_GDBGFIFOSPACE` writer"]
pub type W = crate::W<Usb3GdbgfifospaceSpec>;
#[doc = "Field `FIFO_QUEUE_SELECT` reader - FIFO/Queue Select (or) Port-Select\n\nFIFO/Queue Select\\[8:5\\]
indicates the FIFO/Queue Type\n\nFIFO/Queue Select\\[4:0\\]
indicates the FIFO/Queue Number\n\nPort-Select\\[3:0\\]
selects the port-number when accessing\n\nGDBGLTSSM register."]
pub type FifoQueueSelectR = crate::FieldReader<u16>;
#[doc = "Field `FIFO_QUEUE_SELECT` writer - FIFO/Queue Select (or) Port-Select\n\nFIFO/Queue Select\\[8:5\\]
indicates the FIFO/Queue Type\n\nFIFO/Queue Select\\[4:0\\]
indicates the FIFO/Queue Number\n\nPort-Select\\[3:0\\]
selects the port-number when accessing\n\nGDBGLTSSM register."]
pub type FifoQueueSelectW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SPACE_AVAILABLE` reader - Space Avalible\n\nSpace Avalible"]
pub type SpaceAvailableR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - FIFO/Queue Select (or) Port-Select\n\nFIFO/Queue Select\\[8:5\\]
indicates the FIFO/Queue Type\n\nFIFO/Queue Select\\[4:0\\]
indicates the FIFO/Queue Number\n\nPort-Select\\[3:0\\]
selects the port-number when accessing\n\nGDBGLTSSM register."]
    #[inline(always)]
    pub fn fifo_queue_select(&self) -> FifoQueueSelectR {
        FifoQueueSelectR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:31 - Space Avalible\n\nSpace Avalible"]
    #[inline(always)]
    pub fn space_available(&self) -> SpaceAvailableR {
        SpaceAvailableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FIFO/Queue Select (or) Port-Select\n\nFIFO/Queue Select\\[8:5\\]
indicates the FIFO/Queue Type\n\nFIFO/Queue Select\\[4:0\\]
indicates the FIFO/Queue Number\n\nPort-Select\\[3:0\\]
selects the port-number when accessing\n\nGDBGLTSSM register."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_queue_select(&mut self) -> FifoQueueSelectW<Usb3GdbgfifospaceSpec> {
        FifoQueueSelectW::new(self, 0)
    }
}
#[doc = "Global Debug Queue/FIFO Space Available Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgfifospace::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbgfifospace::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdbgfifospaceSpec;
impl crate::RegisterSpec for Usb3GdbgfifospaceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdbgfifospace::R`](R) reader structure"]
impl crate::Readable for Usb3GdbgfifospaceSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gdbgfifospace::W`](W) writer structure"]
impl crate::Writable for Usb3GdbgfifospaceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GDBGFIFOSPACE to value 0x0042_0000"]
impl crate::Resettable for Usb3GdbgfifospaceSpec {
    const RESET_VALUE: u32 = 0x0042_0000;
}
