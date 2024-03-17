#[doc = "Register `DENALI_CTL_46` reader"]
pub type R = crate::R<DenaliCtl46Spec>;
#[doc = "Register `DENALI_CTL_46` writer"]
pub type W = crate::W<DenaliCtl46Spec>;
#[doc = "Field `ADDRESS_MIRRORING` reader - Indicates which chip selects support address mirroring. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable."]
pub type AddressMirroringR = crate::FieldReader;
#[doc = "Field `ADDRESS_MIRRORING` writer - Indicates which chip selects support address mirroring. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable."]
pub type AddressMirroringW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NO_MEMORY_DM` reader - Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
pub type NoMemoryDmR = crate::BitReader;
#[doc = "Field `NO_MEMORY_DM` writer - Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
pub type NoMemoryDmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Indicates which chip selects support address mirroring. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn address_mirroring(&self) -> AddressMirroringR {
        AddressMirroringR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 24 - Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
    #[inline(always)]
    pub fn no_memory_dm(&self) -> NoMemoryDmR {
        NoMemoryDmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Indicates which chip selects support address mirroring. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn address_mirroring(&mut self) -> AddressMirroringW<DenaliCtl46Spec> {
        AddressMirroringW::new(self, 0)
    }
    #[doc = "Bit 24 - Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
    #[inline(always)]
    #[must_use]
    pub fn no_memory_dm(&mut self) -> NoMemoryDmW<DenaliCtl46Spec> {
        NoMemoryDmW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl46Spec;
impl crate::RegisterSpec for DenaliCtl46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_46::R`](R) reader structure"]
impl crate::Readable for DenaliCtl46Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_46::W`](W) writer structure"]
impl crate::Writable for DenaliCtl46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_46 to value 0"]
impl crate::Resettable for DenaliCtl46Spec {
    const RESET_VALUE: u32 = 0;
}
