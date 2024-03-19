#[doc = "Register `GMAC_MMC_IPC_INT_MSK` reader"]
pub type R = crate::R<GmacMmcIpcIntMskSpec>;
#[doc = "Register `GMAC_MMC_IPC_INT_MSK` writer"]
pub type W = crate::W<GmacMmcIpcIntMskSpec>;
#[doc = "Field `INT0` reader - Setting this bit masks the interrupt when the rxipv4_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - Setting this bit masks the interrupt when the rxipv4_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - Setting this bit masks the interrupt when the rxipv4_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - Setting this bit masks the interrupt when the rxipv4_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT5` reader - Setting this bit masks the interrupt when the rxipv6_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT5` writer - Setting this bit masks the interrupt when the rxipv6_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT6` reader - Setting this bit masks the interrupt when the rxipv6_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int6R = crate::BitReader;
#[doc = "Field `INT6` writer - Setting this bit masks the interrupt when the rxipv6_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT9` reader - Setting this bit masks the interrupt when the rxudp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int9R = crate::BitReader;
#[doc = "Field `INT9` writer - Setting this bit masks the interrupt when the rxudp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT11` reader - Setting this bit masks the interrupt when the rxtcp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int11R = crate::BitReader;
#[doc = "Field `INT11` writer - Setting this bit masks the interrupt when the rxtcp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT13` reader - Setting this bit masks the interrupt when the rxicmp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT13` writer - Setting this bit masks the interrupt when the rxicmp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT17` reader - Setting this bit masks the interrupt when the\n\nrxipv4_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int17R = crate::BitReader;
#[doc = "Field `INT17` writer - Setting this bit masks the interrupt when the\n\nrxipv4_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT22` reader - Setting this bit masks the interrupt when the\n\nrxipv6_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int22R = crate::BitReader;
#[doc = "Field `INT22` writer - Setting this bit masks the interrupt when the\n\nrxipv6_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT25` reader - Setting this bit masks the interrupt when the rxudp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int25R = crate::BitReader;
#[doc = "Field `INT25` writer - Setting this bit masks the interrupt when the rxudp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT27` reader - Setting this bit masks the interrupt when the rxtcp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int27R = crate::BitReader;
#[doc = "Field `INT27` writer - Setting this bit masks the interrupt when the rxtcp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT29` reader - Setting this bit masks the interrupt when the rxicmp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int29R = crate::BitReader;
#[doc = "Field `INT29` writer - Setting this bit masks the interrupt when the rxicmp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int29W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the rxipv4_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the rxipv4_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting this bit masks the interrupt when the rxipv6_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting this bit masks the interrupt when the rxipv6_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting this bit masks the interrupt when the rxudp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Setting this bit masks the interrupt when the rxtcp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting this bit masks the interrupt when the rxicmp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Setting this bit masks the interrupt when the\n\nrxipv4_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int17(&self) -> Int17R {
        Int17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Setting this bit masks the interrupt when the\n\nrxipv6_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int22(&self) -> Int22R {
        Int22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Setting this bit masks the interrupt when the rxudp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int25(&self) -> Int25R {
        Int25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Setting this bit masks the interrupt when the rxtcp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int27(&self) -> Int27R {
        Int27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Setting this bit masks the interrupt when the rxicmp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int29(&self) -> Int29R {
        Int29R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the rxipv4_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<GmacMmcIpcIntMskSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the rxipv4_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<GmacMmcIpcIntMskSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 5 - Setting this bit masks the interrupt when the rxipv6_gd_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<GmacMmcIpcIntMskSpec> {
        Int5W::new(self, 5)
    }
    #[doc = "Bit 6 - Setting this bit masks the interrupt when the rxipv6_hdrerr_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> Int6W<GmacMmcIpcIntMskSpec> {
        Int6W::new(self, 6)
    }
    #[doc = "Bit 9 - Setting this bit masks the interrupt when the rxudp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> Int9W<GmacMmcIpcIntMskSpec> {
        Int9W::new(self, 9)
    }
    #[doc = "Bit 11 - Setting this bit masks the interrupt when the rxtcp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> Int11W<GmacMmcIpcIntMskSpec> {
        Int11W::new(self, 11)
    }
    #[doc = "Bit 13 - Setting this bit masks the interrupt when the rxicmp_err_frms\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<GmacMmcIpcIntMskSpec> {
        Int13W::new(self, 13)
    }
    #[doc = "Bit 17 - Setting this bit masks the interrupt when the\n\nrxipv4_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> Int17W<GmacMmcIpcIntMskSpec> {
        Int17W::new(self, 17)
    }
    #[doc = "Bit 22 - Setting this bit masks the interrupt when the\n\nrxipv6_hdrerr_octets counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> Int22W<GmacMmcIpcIntMskSpec> {
        Int22W::new(self, 22)
    }
    #[doc = "Bit 25 - Setting this bit masks the interrupt when the rxudp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> Int25W<GmacMmcIpcIntMskSpec> {
        Int25W::new(self, 25)
    }
    #[doc = "Bit 27 - Setting this bit masks the interrupt when the rxtcp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> Int27W<GmacMmcIpcIntMskSpec> {
        Int27W::new(self, 27)
    }
    #[doc = "Bit 29 - Setting this bit masks the interrupt when the rxicmp_err_octets\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> Int29W<GmacMmcIpcIntMskSpec> {
        Int29W::new(self, 29)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_ipc_int_msk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_ipc_int_msk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcIpcIntMskSpec;
impl crate::RegisterSpec for GmacMmcIpcIntMskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_ipc_int_msk::R`](R) reader structure"]
impl crate::Readable for GmacMmcIpcIntMskSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_ipc_int_msk::W`](W) writer structure"]
impl crate::Writable for GmacMmcIpcIntMskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_IPC_INT_MSK to value 0"]
impl crate::Resettable for GmacMmcIpcIntMskSpec {
    const RESET_VALUE: u32 = 0;
}
