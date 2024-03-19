#[doc = "Register `DMAC_CR0` reader"]
pub type R = crate::R<DmacCr0Spec>;
#[doc = "Field `DMAC_CR0_BITS_8` reader - Supports peripheral requests:\n\n0 = the DMAC does not provide a peripheral request interface\n\n1 = the DMAC provides the number of peripheral request interfaces\n\nthat the num_periph_req field specifies."]
pub type DmacCr0Bits8R = crate::BitReader;
#[doc = "Field `DMAC_CR0_BITS_7` reader - Indicates the status of the boot_from_pc signal when the DMAC\n\nexited from reset:\n\n0 = boot_from_pc was LOW\n\n1 = boot_from_pc was HIGH"]
pub type DmacCr0Bits7R = crate::BitReader;
#[doc = "Field `DMAC_CR0_BITS_6` reader - Indicates the status of the boot_manager_ns signal when the\n\nDMAC exited from reset:\n\n0 = boot_manager_ns was LOW\n\n1 = boot_manager_ns was HIGH."]
pub type DmacCr0Bits6R = crate::BitReader;
#[doc = "Field `DMAC_CR0_BITS_4` reader - Number of DMA channels that the DMAC supports:\n\nb000 = 1 DMA channel\n\nb001 = 2 DMA channels\n\nb010 = 3 DMA channels\n\n...\n\nb111 = 8 DMA channels."]
pub type DmacCr0Bits4R = crate::FieldReader;
#[doc = "Field `DMAC_CR0_BITS_2` reader - Number of peripheral request interfaces that the DMAC provides:\n\nb00000 = 1 peripheral request interface\n\nb00001 = 2 peripheral request interfaces\n\nb00010 = 3 peripheral request interfaces\n\n...\n\nb11111 = 32 peripheral request interfaces."]
pub type DmacCr0Bits2R = crate::FieldReader;
#[doc = "Field `DMAC_CR0_BITS_1` reader - Number of interrupt outputs that the DMAC provides:\n\nb00000 = 1 interrupt output, irq\\[0\\]\n\nb00001 = 2 interrupt outputs, irq\\[1:0\\]\n\nb00010 = 3 interrupt outputs, irq\\[2:0\\]\n\n...\n\nb11111 = 32 interrupt outputs, irq\\[31:0\\]."]
pub type DmacCr0Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Supports peripheral requests:\n\n0 = the DMAC does not provide a peripheral request interface\n\n1 = the DMAC provides the number of peripheral request interfaces\n\nthat the num_periph_req field specifies."]
    #[inline(always)]
    pub fn dmac_cr0_bits_8(&self) -> DmacCr0Bits8R {
        DmacCr0Bits8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the status of the boot_from_pc signal when the DMAC\n\nexited from reset:\n\n0 = boot_from_pc was LOW\n\n1 = boot_from_pc was HIGH"]
    #[inline(always)]
    pub fn dmac_cr0_bits_7(&self) -> DmacCr0Bits7R {
        DmacCr0Bits7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the status of the boot_manager_ns signal when the\n\nDMAC exited from reset:\n\n0 = boot_manager_ns was LOW\n\n1 = boot_manager_ns was HIGH."]
    #[inline(always)]
    pub fn dmac_cr0_bits_6(&self) -> DmacCr0Bits6R {
        DmacCr0Bits6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Number of DMA channels that the DMAC supports:\n\nb000 = 1 DMA channel\n\nb001 = 2 DMA channels\n\nb010 = 3 DMA channels\n\n...\n\nb111 = 8 DMA channels."]
    #[inline(always)]
    pub fn dmac_cr0_bits_4(&self) -> DmacCr0Bits4R {
        DmacCr0Bits4R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:16 - Number of peripheral request interfaces that the DMAC provides:\n\nb00000 = 1 peripheral request interface\n\nb00001 = 2 peripheral request interfaces\n\nb00010 = 3 peripheral request interfaces\n\n...\n\nb11111 = 32 peripheral request interfaces."]
    #[inline(always)]
    pub fn dmac_cr0_bits_2(&self) -> DmacCr0Bits2R {
        DmacCr0Bits2R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Number of interrupt outputs that the DMAC provides:\n\nb00000 = 1 interrupt output, irq\\[0\\]\n\nb00001 = 2 interrupt outputs, irq\\[1:0\\]\n\nb00010 = 3 interrupt outputs, irq\\[2:0\\]\n\n...\n\nb11111 = 32 interrupt outputs, irq\\[31:0\\]."]
    #[inline(always)]
    pub fn dmac_cr0_bits_1(&self) -> DmacCr0Bits1R {
        DmacCr0Bits1R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
#[doc = "Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCr0Spec;
impl crate::RegisterSpec for DmacCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cr0::R`](R) reader structure"]
impl crate::Readable for DmacCr0Spec {}
#[doc = "`reset()` method sets DMAC_CR0 to value 0"]
impl crate::Resettable for DmacCr0Spec {
    const RESET_VALUE: u32 = 0;
}
