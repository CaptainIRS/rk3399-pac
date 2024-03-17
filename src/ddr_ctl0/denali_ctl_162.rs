#[doc = "Register `DENALI_CTL_162` reader"]
pub type R = crate::R<DenaliCtl162Spec>;
#[doc = "Register `DENALI_CTL_162` writer"]
pub type W = crate::W<DenaliCtl162Spec>;
#[doc = "Field `DFS_ALWAYS_WRITE_FSP` reader - Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
pub type DfsAlwaysWriteFspR = crate::BitReader;
#[doc = "Field `DFS_ALWAYS_WRITE_FSP` writer - Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
pub type DfsAlwaysWriteFspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP_STATUS` reader - Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
pub type FspStatusR = crate::BitReader;
#[doc = "Field `FSP_STATUS` writer - Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
pub type FspStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP_OP_CURRENT` reader - Reports which FSP set the memory is currently using."]
pub type FspOpCurrentR = crate::BitReader;
#[doc = "Field `FSP_OP_CURRENT` writer - Reports which FSP set the memory is currently using."]
pub type FspOpCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP_WR_CURRENT` reader - Reports which FSP set the memory will target with write commands."]
pub type FspWrCurrentR = crate::BitReader;
#[doc = "Field `FSP_WR_CURRENT` writer - Reports which FSP set the memory will target with write commands."]
pub type FspWrCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
    #[inline(always)]
    pub fn dfs_always_write_fsp(&self) -> DfsAlwaysWriteFspR {
        DfsAlwaysWriteFspR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
    #[inline(always)]
    pub fn fsp_status(&self) -> FspStatusR {
        FspStatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Reports which FSP set the memory is currently using."]
    #[inline(always)]
    pub fn fsp_op_current(&self) -> FspOpCurrentR {
        FspOpCurrentR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Reports which FSP set the memory will target with write commands."]
    #[inline(always)]
    pub fn fsp_wr_current(&self) -> FspWrCurrentR {
        FspWrCurrentR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_always_write_fsp(&mut self) -> DfsAlwaysWriteFspW<DenaliCtl162Spec> {
        DfsAlwaysWriteFspW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_status(&mut self) -> FspStatusW<DenaliCtl162Spec> {
        FspStatusW::new(self, 8)
    }
    #[doc = "Bit 16 - Reports which FSP set the memory is currently using."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_op_current(&mut self) -> FspOpCurrentW<DenaliCtl162Spec> {
        FspOpCurrentW::new(self, 16)
    }
    #[doc = "Bit 24 - Reports which FSP set the memory will target with write commands."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_wr_current(&mut self) -> FspWrCurrentW<DenaliCtl162Spec> {
        FspWrCurrentW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_162::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_162::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl162Spec;
impl crate::RegisterSpec for DenaliCtl162Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_162::R`](R) reader structure"]
impl crate::Readable for DenaliCtl162Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_162::W`](W) writer structure"]
impl crate::Writable for DenaliCtl162Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_162 to value 0"]
impl crate::Resettable for DenaliCtl162Spec {
    const RESET_VALUE: u32 = 0;
}
