#[doc = "Register `MMC_IPC_INTR` reader"]
pub type R = crate::R<MmcIpcIntrSpec>;
#[doc = "Field `INT0` reader - The bit is set when the rxipv4_gd_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT1` reader - The bit is set when the rxipv4_hdrerr_frms counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT5` reader - The bit is set when the rxipv6_gd_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT6` reader - The bit is set when the rxipv6_hdrerr_frms counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int6R = crate::BitReader;
#[doc = "Field `INT9` reader - The bit is set when the rxudp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int9R = crate::BitReader;
#[doc = "Field `INT11` reader - The bit is set when the rxtcp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int11R = crate::BitReader;
#[doc = "Field `INT13` reader - The bit is set when the rxicmp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT17` reader - The bit is set when the rxipv4_hdrerr_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int17R = crate::BitReader;
#[doc = "Field `INT22` reader - The bit is set when the rxipv6_hdrerr_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int22R = crate::BitReader;
#[doc = "Field `INT25` reader - The bit is set when the rxudp_err_octets counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int25R = crate::BitReader;
#[doc = "Field `INT27` reader - The bit is set when the rxtcp_err_octets counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int27R = crate::BitReader;
#[doc = "Field `INT29` reader - The bit is set when the rxicmp_err_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int29R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is set when the rxipv4_gd_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is set when the rxipv4_hdrerr_frms counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is set when the rxipv6_gd_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is set when the rxipv6_hdrerr_frms counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is set when the rxudp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is set when the rxtcp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is set when the rxicmp_err_frms counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is set when the rxipv4_hdrerr_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int17(&self) -> Int17R {
        Int17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is set when the rxipv6_hdrerr_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int22(&self) -> Int22R {
        Int22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - The bit is set when the rxudp_err_octets counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int25(&self) -> Int25R {
        Int25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - The bit is set when the rxtcp_err_octets counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int27(&self) -> Int27R {
        Int27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - The bit is set when the rxicmp_err_octets counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int29(&self) -> Int29R {
        Int29R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_intr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIpcIntrSpec;
impl crate::RegisterSpec for MmcIpcIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_intr::R`](R) reader structure"]
impl crate::Readable for MmcIpcIntrSpec {}
#[doc = "`reset()` method sets MMC_IPC_INTR to value 0"]
impl crate::Resettable for MmcIpcIntrSpec {
    const RESET_VALUE: u32 = 0;
}
