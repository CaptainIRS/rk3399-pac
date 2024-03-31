#[doc = "Register `BASIC_STATUS1` reader"]
pub type R = crate::R<BasicStatus1Spec>;
#[doc = "Field `SYS_PAGE_SIZE` reader - System page size\n\nThese bits reflect the setting of the System Page Size Register in\n\nthe SR IOV capability of each PF. Bits \\[15:0\\]
reflect bits \\[15:0\\]
of\n\nSystem Page Size register of PF0"]
pub type SysPageSizeR = crate::FieldReader<u16>;
#[doc = "Field `FC_ST` reader - Function status\n\nThese outputs indicate the states of the Command Register bits\n\nin the PCI configuration space of each Function. These outputs\n\nare used to enable requests and completions from the host logic.\n\nThe assignment of bits is as follows:\n\nBit 0: Function 0 IO Space Enable\n\nBit 1: Function 0 Memory Space Enable\n\nBit 2: Function 0 Bus Master Enable\n\nBit 3: Function 0 INTx Disable\n\nand so on depending on the number of functions."]
pub type FcStR = crate::FieldReader;
#[doc = "Field `LINK_ST` reader - Link status\n\nStatus of the PCI Express link.\n\n2'b00 = No receivers detected.\n\n2'b01 = Link training in progress.\n\n2'b10 = Link up , DL initialization in progress.\n\n2'b11 = Link up, DL initialization completed."]
pub type LinkStR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - System page size\n\nThese bits reflect the setting of the System Page Size Register in\n\nthe SR IOV capability of each PF. Bits \\[15:0\\]
reflect bits \\[15:0\\]
of\n\nSystem Page Size register of PF0"]
    #[inline(always)]
    pub fn sys_page_size(&self) -> SysPageSizeR {
        SysPageSizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Function status\n\nThese outputs indicate the states of the Command Register bits\n\nin the PCI configuration space of each Function. These outputs\n\nare used to enable requests and completions from the host logic.\n\nThe assignment of bits is as follows:\n\nBit 0: Function 0 IO Space Enable\n\nBit 1: Function 0 Memory Space Enable\n\nBit 2: Function 0 Bus Master Enable\n\nBit 3: Function 0 INTx Disable\n\nand so on depending on the number of functions."]
    #[inline(always)]
    pub fn fc_st(&self) -> FcStR {
        FcStR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Link status\n\nStatus of the PCI Express link.\n\n2'b00 = No receivers detected.\n\n2'b01 = Link training in progress.\n\n2'b10 = Link up , DL initialization in progress.\n\n2'b11 = Link up, DL initialization completed."]
    #[inline(always)]
    pub fn link_st(&self) -> LinkStR {
        LinkStR::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "Basic status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BasicStatus1Spec;
impl crate::RegisterSpec for BasicStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`basic_status1::R`](R) reader structure"]
impl crate::Readable for BasicStatus1Spec {}
#[doc = "`reset()` method sets BASIC_STATUS1 to value 0x0008_0001"]
impl crate::Resettable for BasicStatus1Spec {
    const RESET_VALUE: u32 = 0x0008_0001;
}
