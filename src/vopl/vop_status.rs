#[doc = "Register `VOP_STATUS` reader"]
pub type R = crate::R<VopStatusSpec>;
#[doc = "Register `VOP_STATUS` writer"]
pub type W = crate::W<VopStatusSpec>;
#[doc = "Field `DSP_VCNT` reader - read the dsp vertical counter"]
pub type DspVcntR = crate::FieldReader<u16>;
#[doc = "Field `MMU_IDLE` reader - mmu idle status"]
pub type MmuIdleR = crate::BitReader;
#[doc = "Field `MMU_IDLE` writer - mmu idle status"]
pub type MmuIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_STOP_VALID` reader - dma stop valid"]
pub type DmaStopValidR = crate::BitReader;
#[doc = "Field `DMA_STOP_VALID` writer - dma stop valid"]
pub type DmaStopValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - read the dsp vertical counter"]
    #[inline(always)]
    pub fn dsp_vcnt(&self) -> DspVcntR {
        DspVcntR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - mmu idle status"]
    #[inline(always)]
    pub fn mmu_idle(&self) -> MmuIdleR {
        MmuIdleR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dma stop valid"]
    #[inline(always)]
    pub fn dma_stop_valid(&self) -> DmaStopValidR {
        DmaStopValidR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - mmu idle status"]
    #[inline(always)]
    #[must_use]
    pub fn mmu_idle(&mut self) -> MmuIdleW<VopStatusSpec> {
        MmuIdleW::new(self, 16)
    }
    #[doc = "Bit 17 - dma stop valid"]
    #[inline(always)]
    #[must_use]
    pub fn dma_stop_valid(&mut self) -> DmaStopValidW<VopStatusSpec> {
        DmaStopValidW::new(self, 17)
    }
}
#[doc = "vop status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vop_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vop_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VopStatusSpec;
impl crate::RegisterSpec for VopStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vop_status::R`](R) reader structure"]
impl crate::Readable for VopStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`vop_status::W`](W) writer structure"]
impl crate::Writable for VopStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VOP_STATUS to value 0"]
impl crate::Resettable for VopStatusSpec {
    const RESET_VALUE: u32 = 0;
}
