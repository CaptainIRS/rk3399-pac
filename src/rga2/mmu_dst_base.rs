#[doc = "Register `MMU_DST_BASE` reader"]
pub type R = crate::R<MmuDstBaseSpec>;
#[doc = "Register `MMU_DST_BASE` writer"]
pub type W = crate::W<MmuDstBaseSpec>;
#[doc = "Field `SW_MMU_DST_BASE` reader - RGA destination MMU TLB base address (128-bit)"]
pub type SwMmuDstBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_MMU_DST_BASE` writer - RGA destination MMU TLB base address (128-bit)"]
pub type SwMmuDstBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - RGA destination MMU TLB base address (128-bit)"]
    #[inline(always)]
    pub fn sw_mmu_dst_base(&self) -> SwMmuDstBaseR {
        SwMmuDstBaseR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - RGA destination MMU TLB base address (128-bit)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mmu_dst_base(&mut self) -> SwMmuDstBaseW<MmuDstBaseSpec> {
        SwMmuDstBaseW::new(self, 0)
    }
}
#[doc = "RGA destination MMU TLB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_dst_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_dst_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuDstBaseSpec;
impl crate::RegisterSpec for MmuDstBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_dst_base::R`](R) reader structure"]
impl crate::Readable for MmuDstBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_dst_base::W`](W) writer structure"]
impl crate::Writable for MmuDstBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_DST_BASE to value 0"]
impl crate::Resettable for MmuDstBaseSpec {
    const RESET_VALUE: u32 = 0;
}
