#[doc = "Register `MMU_CMD_BASE` reader"]
pub type R = crate::R<MmuCmdBaseSpec>;
#[doc = "Register `MMU_CMD_BASE` writer"]
pub type W = crate::W<MmuCmdBaseSpec>;
#[doc = "Field `SW_MMU_CMD_BASE` reader - RGA command MMU TLB base address (word)"]
pub type SwMmuCmdBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_MMU_CMD_BASE` writer - RGA command MMU TLB base address (word)"]
pub type SwMmuCmdBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - RGA command MMU TLB base address (word)"]
    #[inline(always)]
    pub fn sw_mmu_cmd_base(&self) -> SwMmuCmdBaseR {
        SwMmuCmdBaseR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - RGA command MMU TLB base address (word)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mmu_cmd_base(&mut self) -> SwMmuCmdBaseW<MmuCmdBaseSpec> {
        SwMmuCmdBaseW::new(self, 0)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_cmd_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_cmd_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuCmdBaseSpec;
impl crate::RegisterSpec for MmuCmdBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_cmd_base::R`](R) reader structure"]
impl crate::Readable for MmuCmdBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_cmd_base::W`](W) writer structure"]
impl crate::Writable for MmuCmdBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_CMD_BASE to value 0"]
impl crate::Resettable for MmuCmdBaseSpec {
    const RESET_VALUE: u32 = 0;
}
