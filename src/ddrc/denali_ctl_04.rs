#[doc = "Register `DENALI_CTL_04` reader"]
pub type R = crate::R<DenaliCtl04Spec>;
#[doc = "Register `DENALI_CTL_04` writer"]
pub type W = crate::W<DenaliCtl04Spec>;
#[doc = "Field `DENALI0_WRCMD_SIDE_FIFO_LOG2_DEPTH` reader - Reports the depth of the DENALI port 0 processing FIFO. Value is the log2 value of the depth."]
pub type Denali0WrcmdSideFifoLog2DepthR = crate::FieldReader;
#[doc = "Field `DFS_CLOSE_BANKS` reader - Close all pages before doing DFS. Set to 1 to enable."]
pub type DfsCloseBanksR = crate::BitReader;
#[doc = "Field `DFS_CLOSE_BANKS` writer - Close all pages before doing DFS. Set to 1 to enable."]
pub type DfsCloseBanksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reports the depth of the DENALI port 0 processing FIFO. Value is the log2 value of the depth."]
    #[inline(always)]
    pub fn denali0_wrcmd_side_fifo_log2_depth(&self) -> Denali0WrcmdSideFifoLog2DepthR {
        Denali0WrcmdSideFifoLog2DepthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Close all pages before doing DFS. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_close_banks(&self) -> DfsCloseBanksR {
        DfsCloseBanksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Close all pages before doing DFS. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_close_banks(&mut self) -> DfsCloseBanksW<DenaliCtl04Spec> {
        DfsCloseBanksW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_04::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_04::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl04Spec;
impl crate::RegisterSpec for DenaliCtl04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_04::R`](R) reader structure"]
impl crate::Readable for DenaliCtl04Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_04::W`](W) writer structure"]
impl crate::Writable for DenaliCtl04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_04 to value 0"]
impl crate::Resettable for DenaliCtl04Spec {
    const RESET_VALUE: u32 = 0;
}
