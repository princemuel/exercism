// URL.createObjectURL(e.target.files[0])
const getImageUrl = (src) => {
  let imageUrl = src;
  let response = URL.createObjectURL(imageUrl);
  return response;
};

export default getImageUrl;
